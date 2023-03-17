#![cfg(test)]
extern crate std;

use ed25519_dalek::Keypair;
use ed25519_dalek::Signer;
use rand::thread_rng;
use soroban_auth::{testutils::EnvAuthUtils, AuthorizationContext};
use soroban_sdk::RawVal;
use soroban_sdk::{testutils::BytesN as _, vec, BytesN, Env, IntoVal, Symbol};

use crate::AccError;
use crate::{AccountContract, AccountContractClient, Signature};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn signer_public_key(e: &Env, signer: &Keypair) -> BytesN<32> {
    signer.public.to_bytes().into_val(e)
}

fn create_account_contract(e: &Env) -> AccountContractClient {
    AccountContractClient::new(e, &e.register_contract(None, AccountContract {}))
}

fn sign(e: &Env, signer: &Keypair, payload: &BytesN<32>) -> RawVal {
    Signature {
        public_key: signer_public_key(e, signer),
        signature: signer
            .sign(payload.to_array().as_slice())
            .to_bytes()
            .into_val(e),
    }
    .into_val(e)
}

fn token_auth_context(
    e: &Env,
    token_id: &BytesN<32>,
    fn_name: Symbol,
    amount: i128,
) -> AuthorizationContext {
    AuthorizationContext {
        contract: token_id.clone(),
        fn_name,
        args: ((), (), amount).into_val(e),
    }
}

#[test]
fn test_token_auth() {
    let env: Env = Default::default();

    let account_contract = create_account_contract(&env);

    let mut signers = [generate_keypair(), generate_keypair()];
    if signers[0].public.as_bytes() > signers[1].public.as_bytes() {
        signers.swap(0, 1);
    }
    account_contract.init(&vec![
        &env,
        signer_public_key(&env, &signers[0]),
        signer_public_key(&env, &signers[1]),
    ]);

    let payload = BytesN::random(&env);
    let token = BytesN::random(&env);
    // `__check_auth` can't be called directly, hence we need to use
    // `invoke_account_contract_check_auth` testing utility that emulates being
    // called by the Soroban host during a `require_auth` call.
    env.invoke_account_contract_check_auth::<AccError>(
        &account_contract.contract_id,
        &payload,
        &vec![&env, sign(&env, &signers[0], &payload)],
        &vec![
            &env,
            token_auth_context(&env, &token, Symbol::new(&env, "xfer"), 1000),
        ],
    )
    .unwrap();
    env.invoke_account_contract_check_auth::<AccError>(
        &account_contract.contract_id,
        &payload,
        &vec![&env, sign(&env, &signers[0], &payload)],
        &vec![
            &env,
            token_auth_context(&env, &token, Symbol::new(&env, "xfer"), 1000),
        ],
    )
    .unwrap();

    // Add a spend limit of 1000 per 1 signer.
    account_contract.add_limit(&token, &1000);
    // Verify that this call needs to be authorized.
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            account_contract.address(),
            account_contract.contract_id.clone(),
            Symbol::short("add_limit"),
            (token.clone(), 1000_i128).into_val(&env),
        )]
    );

    // 1 signer no longer can perform the token operation that transfers more
    // than 1000 units.
    assert_eq!(
        env.invoke_account_contract_check_auth::<AccError>(
            &account_contract.contract_id,
            &payload,
            &vec![&env, sign(&env, &signers[0], &payload)],
            &vec![
                &env,
                token_auth_context(&env, &token, Symbol::new(&env, "xfer"), 1001)
            ],
        )
        .err()
        .unwrap()
        .unwrap(),
        AccError::NotEnoughSigners
    );
    assert_eq!(
        env.invoke_account_contract_check_auth::<AccError>(
            &account_contract.contract_id,
            &payload,
            &vec![&env, sign(&env, &signers[0], &payload)],
            &vec![
                &env,
                token_auth_context(&env, &token, Symbol::new(&env, "incr_allow"), 1001)
            ],
        )
        .err()
        .unwrap()
        .unwrap(),
        AccError::NotEnoughSigners
    );

    // 1 signer can still transfer 1000 units.
    env.invoke_account_contract_check_auth::<AccError>(
        &account_contract.contract_id,
        &payload,
        &vec![&env, sign(&env, &signers[0], &payload)],
        &vec![
            &env,
            token_auth_context(&env, &token, Symbol::new(&env, "incr_allow"), 1000),
        ],
    )
    .unwrap();
    // 2 signers can transfer any amount of token.
    env.invoke_account_contract_check_auth::<AccError>(
        &account_contract.contract_id,
        &payload,
        &vec![
            &env,
            sign(&env, &signers[0], &payload),
            sign(&env, &signers[1], &payload),
        ],
        &vec![
            &env,
            token_auth_context(&env, &token, Symbol::new(&env, "xfer"), 10000),
        ],
    )
    .unwrap();
}
