#![cfg(test)]
extern crate std;

use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};

use soroban_sdk::xdr::{
    HashIdPreimage, HashIdPreimageSorobanAuthorizationWithAddress, InvokeContractArgs, Limits,
    ScAddress, ScVal, SorobanAddressCredentials, SorobanAddressCredentialsWithDelegates,
    SorobanAuthorizationEntry, SorobanAuthorizedFunction, SorobanAuthorizedInvocation,
    SorobanCredentials, SorobanDelegateSignature, StringM, VecM, WriteXdr,
};
use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contractimpl,
    crypto::Hash,
    vec, Address, BytesN, Env, Symbol, TryFromVal, Vec,
};

use crate::{record_authorized_calls, DataKey, ModularAccount};

// An account that performs ed25519 verification and is used as a delegate
// signer of `ModularAccount`. Any address type (G- or C-) that implements
// `CustomAccountInterface` can be a delegate, so this is defined here as a
// test fixture rather than as a deployable contract of this crate.
#[contract]
pub struct DelegateAccount;

#[contractimpl]
impl DelegateAccount {
    pub fn __constructor(env: Env, public_key: BytesN<32>) {
        env.storage()
            .instance()
            .set(&DataKey::PublicKey, &public_key);
    }
}

#[contractimpl]
impl CustomAccountInterface for DelegateAccount {
    type Signature = BytesN<64>;
    type Error = soroban_sdk::Error;

    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signature: BytesN<64>,
        auth_contexts: Vec<Context>,
    ) -> Result<(), soroban_sdk::Error> {
        let public_key: BytesN<32> = env.storage().instance().get(&DataKey::PublicKey).unwrap();
        env.crypto()
            .ed25519_verify(&public_key, &signature_payload.into(), &signature);
        record_authorized_calls(&env, &auth_contexts);
        Ok(())
    }
}

// A contract with an operation that requires the account's authorization.
#[contract]
pub struct Protected;

#[contractimpl]
impl Protected {
    pub fn protected(account: Address) {
        account.require_auth();
    }
}

fn sign(env: &Env, key: &SigningKey, payload: &[u8; 32]) -> ScVal {
    let sig: [u8; 64] = key.sign(payload).to_bytes();
    ScVal::try_from_val(env, &BytesN::from_array(env, &sig).to_val()).unwrap()
}

#[test]
fn test_delegate_auth() {
    let env = Env::default();

    let account_key = SigningKey::from_bytes(&[1u8; 32]);
    let key_a = SigningKey::from_bytes(&[2u8; 32]);
    let key_b = SigningKey::from_bytes(&[3u8; 32]);

    let delegate_a = env.register(
        DelegateAccount,
        (BytesN::from_array(&env, &key_a.verifying_key().to_bytes()),),
    );
    let delegate_b = env.register(
        DelegateAccount,
        (BytesN::from_array(&env, &key_b.verifying_key().to_bytes()),),
    );

    // Register the account with its own key and both delegates as signers.
    let account = env.register(
        ModularAccount,
        (
            BytesN::from_array(&env, &account_key.verifying_key().to_bytes()),
            vec![&env, delegate_a.clone(), delegate_b.clone()],
        ),
    );
    let protected = env.register(Protected, ());

    let account_addr: ScAddress = account.clone().into();

    let nonce = 123;
    let signature_expiration_ledger = 100;

    // Create authorized invocation for the `protected` call.
    let root_invocation = SorobanAuthorizedInvocation {
        function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
            contract_address: protected.clone().into(),
            function_name: StringM::try_from("protected").unwrap().into(),
            args: std::vec![ScVal::Address(account_addr.clone())]
                .try_into()
                .unwrap(),
        }),
        sub_invocations: VecM::default(),
    };

    // Build the signature payload by hashing the
    // `HashIdPreimage::SorobanAuthorizationWithAddress` preimage required for
    // `AddressWithDelegates` credentials.
    let network_id = env.ledger().network_id();
    let preimage = HashIdPreimage::SorobanAuthorizationWithAddress(
        HashIdPreimageSorobanAuthorizationWithAddress {
            network_id: network_id.to_array().into(),
            nonce,
            signature_expiration_ledger,
            invocation: root_invocation.clone(),
            address: account_addr.clone(),
        },
    );
    let preimage_xdr = preimage.to_xdr(Limits::none()).unwrap();
    let payload: [u8; 32] = Sha256::digest(&preimage_xdr).into();

    // Negative scenario: attach a delegate signer that isn't registered with the
    // account. The account should reject the authorization with an
    // `UnknownDelegate` error.
    let unknown_key = SigningKey::from_bytes(&[4u8; 32]);
    let unknown_delegate = env.register(
        DelegateAccount,
        (BytesN::from_array(
            &env,
            &unknown_key.verifying_key().to_bytes(),
        ),),
    );
    let mut bad_delegates = std::vec![
        SorobanDelegateSignature {
            address: delegate_a.clone().into(),
            signature: sign(&env, &key_a, &payload),
            nested_delegates: VecM::default(),
        },
        SorobanDelegateSignature {
            address: unknown_delegate.clone().into(),
            signature: sign(&env, &unknown_key, &payload),
            nested_delegates: VecM::default(),
        },
    ];
    // Delegates must be sorted by address.
    bad_delegates.sort_by(|x, y| x.address.cmp(&y.address));
    env.set_auths(&[SorobanAuthorizationEntry {
        credentials: SorobanCredentials::AddressWithDelegates(
            SorobanAddressCredentialsWithDelegates {
                address_credentials: SorobanAddressCredentials {
                    address: account_addr.clone(),
                    nonce,
                    signature_expiration_ledger,
                    signature: sign(&env, &account_key, &payload),
                },
                delegates: bad_delegates.try_into().unwrap(),
            },
        ),
        root_invocation: root_invocation.clone(),
    }]);
    // The call will fail due to the auth failure.
    assert!(ProtectedClient::new(&env, &protected)
        .try_protected(&account)
        .is_err());

    // Positive scenario: each registered delegate signs the same payload with
    // its own distinct key.
    let mut delegates = std::vec![
        SorobanDelegateSignature {
            address: delegate_a.clone().into(),
            signature: sign(&env, &key_a, &payload),
            nested_delegates: VecM::default(),
        },
        SorobanDelegateSignature {
            address: delegate_b.clone().into(),
            signature: sign(&env, &key_b, &payload),
            nested_delegates: VecM::default(),
        },
    ];
    // Delegates must be sorted by address.
    delegates.sort_by(|x, y| x.address.cmp(&y.address));

    // Build the full authorization entry with `AddressWithDelegates`
    // credentials containing both delegates.
    env.set_auths(&[SorobanAuthorizationEntry {
        credentials: SorobanCredentials::AddressWithDelegates(
            SorobanAddressCredentialsWithDelegates {
                address_credentials: SorobanAddressCredentials {
                    address: account_addr.clone(),
                    nonce,
                    signature_expiration_ledger,
                    // Also include the account's own signature in the
                    // credentials, as it is required by `ModularAccount`'s own
                    // verification step.
                    signature: sign(&env, &account_key, &payload),
                },
                delegates: delegates.try_into().unwrap(),
            },
        ),
        root_invocation: root_invocation.clone(),
    }]);

    // Call the `protected` function with the enforced authorization payload
    // above.
    //
    // Note that testing delegated auth via
    // `env.try_invoke_contract_check_auth` is not supported at the moment, so
    // only `set_auths` plus a wrapper call can be used to test the full flow.
    ProtectedClient::new(&env, &protected).protected(&account);

    // Both the account and its delegates observe a single call to `protected`
    // in their authorization contexts.
    let expected = vec![&env, Symbol::new(&env, "protected")];
    for addr in [&account, &delegate_a, &delegate_b] {
        let calls: Vec<Symbol> = env.as_contract(addr, || {
            env.storage()
                .instance()
                .get(&DataKey::AuthorizedCalls)
                .unwrap()
        });
        assert_eq!(calls, expected);
    }
}
