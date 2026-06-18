#![cfg(test)]
extern crate std;

use soroban_sdk::xdr::{
    InvokeContractArgs, ScAddress, ScVal, SorobanAddressCredentials,
    SorobanAddressCredentialsWithDelegates, SorobanAuthorizationEntry, SorobanAuthorizedFunction,
    SorobanAuthorizedInvocation, SorobanCredentials, SorobanDelegateSignature, StringM, VecM,
};
use soroban_sdk::{
    auth::{Context, ContractContext, CustomAccountInterface},
    contract, contractimpl, contracttype,
    crypto::Hash,
    testutils::{AuthorizedFunction, AuthorizedInvocation},
    vec, Address, Env, IntoVal, Symbol, Vec,
};

use crate::{Error, ModularAccount};

#[contracttype]
enum DelegateAccountDataKey {
    // Records the contexts the delegate approved so the test can verify
    // the delegation reached it.
    ApprovedContexts,
}

// A simple account that the ModularAccount can delegate to for auth.
//
// It will always authorize an auth request, and store a copy of the auth
// context for later comparing in tests.
#[contract]
pub struct DelegateAccount;

#[contractimpl]
impl CustomAccountInterface for DelegateAccount {
    type Signature = ();
    type Error = Error;
    fn __check_auth(
        env: Env,
        _signature_payload: Hash<32>,
        _signatures: (),
        auth_contexts: Vec<Context>,
    ) -> Result<(), Error> {
        env.storage()
            .instance()
            .set(&DelegateAccountDataKey::ApprovedContexts, &auth_contexts);
        // Returning `Ok(())` approves the auth;
        // returning an error would reject it.
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

#[test]
fn test() {
    let env = Env::default();
    let delegate = env.register(DelegateAccount, ());
    // Register the modular account with `delegate` as an allowed signer.
    let account = env.register(ModularAccount, (vec![&env, delegate.clone()],));
    let protected = env.register(Protected, ());

    let account_addr: ScAddress = account.clone().try_into().unwrap();
    let delegate_addr: ScAddress = delegate.clone().try_into().unwrap();

    // This authorization entry is normally built by the user's
    // wallet/tooling and attached to the transaction. It authorizes
    // `protected` on behalf of the account, and attaches `delegate` as a
    // delegated signer. Delegates must be sorted by address with no
    // duplicates.
    env.set_auths(&[SorobanAuthorizationEntry {
        credentials: SorobanCredentials::AddressWithDelegates(
            SorobanAddressCredentialsWithDelegates {
                address_credentials: SorobanAddressCredentials {
                    address: account_addr.clone(),
                    nonce: 1,
                    signature_expiration_ledger: 100,
                    // The account verifies no signature of its own.
                    signature: ScVal::Void,
                },
                delegates: std::vec![SorobanDelegateSignature {
                    address: delegate_addr,
                    signature: ScVal::Void,
                    nested_delegates: VecM::default(),
                }]
                .try_into()
                .unwrap(),
            },
        ),
        root_invocation: SorobanAuthorizedInvocation {
            function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                contract_address: protected.clone().try_into().unwrap(),
                function_name: StringM::try_from("protected").unwrap().into(),
                args: std::vec![ScVal::Address(account_addr)].try_into().unwrap(),
            }),
            sub_invocations: VecM::default(),
        },
    }]);

    // The call succeeds: the account delegates its authentication to
    // `delegate`, which approves it.
    ProtectedClient::new(&env, &protected).protected(&account);

    // The account authorized the `protected` call. Delegating to
    // `delegate` is not recorded as a separate authorization.
    assert_eq!(
        env.auths(),
        std::vec![(
            account.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    protected.clone(),
                    Symbol::new(&env, "protected"),
                    (account.clone(),).into_val(&env),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );

    // The delegation actually reached `delegate`, which approved the
    // same invocation that was authorized above.
    let approved: Vec<Context> = env.as_contract(&delegate, || {
        env.storage()
            .instance()
            .get(&DelegateAccountDataKey::ApprovedContexts)
            .unwrap()
    });
    // `Context` does not implement `Debug` in soroban-sdk 27.0.0-rc.1, so this
    // compares with `==` instead of `assert_eq!`.
    assert!(
        approved
            == vec![
                &env,
                Context::Contract(ContractContext {
                    contract: protected.clone(),
                    fn_name: Symbol::new(&env, "protected"),
                    args: (account.clone(),).into_val(&env),
                }),
            ],
    );
}
