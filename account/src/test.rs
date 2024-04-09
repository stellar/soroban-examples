#![cfg(test)]
extern crate std;

use ed25519_dalek::Keypair;
use ed25519_dalek::Signer;
use rand::thread_rng;
use soroban_sdk::auth::ContractContext;
use soroban_sdk::symbol_short;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::testutils::AuthorizedFunction;
use soroban_sdk::testutils::AuthorizedInvocation;
use soroban_sdk::Val;
use soroban_sdk::{
    auth::Context, testutils::BytesN as _, vec, Address, BytesN, Env, IntoVal, Symbol,
};

use crate::AccError;
use crate::{AccSignature, AccountContract, AccountContractClient};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn signer_public_key(e: &Env, signer: &Keypair) -> BytesN<32> {
    signer.public.to_bytes().into_val(e)
}

fn create_account_contract(e: &Env) -> AccountContractClient {
    AccountContractClient::new(e, &e.register_contract(None, AccountContract {}))
}

fn sign(e: &Env, signer: &Keypair, payload: &BytesN<32>) -> Val {
    AccSignature {
        public_key: signer_public_key(e, signer),
        signature: signer
            .sign(payload.to_array().as_slice())
            .to_bytes()
            .into_val(e),
    }
    .into_val(e)
}

fn token_auth_context(e: &Env, token_id: &Address, fn_name: Symbol, amount: i128) -> Context {
    Context::Contract(ContractContext {
        contract: token_id.clone(),
        fn_name,
        args: ((), (), amount).into_val(e),
    })
}

#[test]
fn test_token_auth() {
    let env = Env::default();
    env.mock_all_auths();

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
    let token = Address::generate(&env);
    // `__check_auth` can't be called directly, hence we need to use
    // `try_invoke_contract_check_auth` testing utility that emulates being
    // called by the Soroban host during a `require_auth` call.
    env.try_invoke_contract_check_auth::<AccError>(
        &account_contract.address,
        &payload,
        vec![&env, sign(&env, &signers[0], &payload)].into(),
        &vec![
            &env,
            token_auth_context(&env, &token, Symbol::new(&env, "transfer"), 1000),
        ],
    )
    .unwrap();
    env.try_invoke_contract_check_auth::<AccError>(
        &account_contract.address,
        &payload,
        vec![&env, sign(&env, &signers[0], &payload)].into(),
        &vec![
            &env,
            token_auth_context(&env, &token, Symbol::new(&env, "transfer"), 1000),
        ],
    )
    .unwrap();

    // Add a spend limit of 1000 per 1 signer.
    account_contract.add_limit(&token, &1000);
    // Verify that this call needs to be authorized.

    assert_eq!(
        env.auths(),
        std::vec![(
            account_contract.address.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    account_contract.address.clone(),
                    symbol_short!("add_limit"),
                    (token.clone(), 1000_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // 1 signer no longer can perform the token operation that transfers more
    // than 1000 units.
    assert_eq!(
        env.try_invoke_contract_check_auth::<AccError>(
            &account_contract.address,
            &payload,
            vec![&env, sign(&env, &signers[0], &payload)].into(),
            &vec![
                &env,
                token_auth_context(&env, &token, Symbol::new(&env, "transfer"), 1001)
            ],
        )
        .err()
        .unwrap()
        .unwrap(),
        AccError::NotEnoughSigners
    );
    assert_eq!(
        env.try_invoke_contract_check_auth::<AccError>(
            &account_contract.address,
            &payload,
            vec![&env, sign(&env, &signers[0], &payload)].into(),
            &vec![
                &env,
                token_auth_context(&env, &token, Symbol::new(&env, "approve"), 1001)
            ],
        )
        .err()
        .unwrap()
        .unwrap(),
        AccError::NotEnoughSigners
    );

    // 1 signer can still transfer 1000 units.
    env.try_invoke_contract_check_auth::<AccError>(
        &account_contract.address,
        &payload,
        vec![&env, sign(&env, &signers[0], &payload)].into(),
        &vec![
            &env,
            token_auth_context(&env, &token, Symbol::new(&env, "approve"), 1000),
        ],
    )
    .unwrap();
    // 2 signers can transfer any amount of token.
    env.try_invoke_contract_check_auth::<AccError>(
        &account_contract.address,
        &payload,
        vec![
            &env,
            sign(&env, &signers[0], &payload),
            sign(&env, &signers[1], &payload),
        ]
        .into(),
        &vec![
            &env,
            token_auth_context(&env, &token, Symbol::new(&env, "transfer"), 10000),
        ],
    )
    .unwrap();
}
