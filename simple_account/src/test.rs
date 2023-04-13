#![cfg(test)]
extern crate std;

use ed25519_dalek::Keypair;
use ed25519_dalek::Signer;
use rand::thread_rng;
use soroban_auth::testutils::EnvAuthUtils;
use soroban_sdk::RawVal;
use soroban_sdk::Status;
use soroban_sdk::{testutils::BytesN as _, vec, BytesN, Env, IntoVal};

use crate::SimpleAccount;
use crate::SimpleAccountClient;

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn create_account_contract(e: &Env) -> SimpleAccountClient {
    SimpleAccountClient::new(e, &e.register_contract(None, SimpleAccount {}))
}

fn sign(e: &Env, signer: &Keypair, payload: &BytesN<32>) -> RawVal {
    let signature: BytesN<64> = signer
        .sign(payload.to_array().as_slice())
        .to_bytes()
        .into_val(e);
    signature.into_val(e)
}

#[test]
fn test_account() {
    let env: Env = Default::default();

    let account_contract = create_account_contract(&env);

    let signer = generate_keypair();
    account_contract.init(&signer.public.to_bytes().into_val(&env));

    let payload = BytesN::random(&env);
    // `__check_auth` can't be called directly, hence we need to use
    // `invoke_account_contract_check_auth` testing utility that emulates being
    // called by the Soroban host during a `require_auth` call.
    env.invoke_account_contract_check_auth::<Status>(
        &account_contract.contract_id,
        &payload,
        &vec![&env, sign(&env, &signer, &payload)],
        &vec![&env],
    )
    // Unwrap the result to make sure there is no error.
    .unwrap();

    // Now pass a random bytes array instead of the signature - this should
    // result in an error as this is not a valid signature.
    assert!(env
        .invoke_account_contract_check_auth::<Status>(
            &account_contract.contract_id,
            &payload,
            &vec![&env, BytesN::<64>::random(&env).into()],
            &vec![&env],
        )
        .is_err());
}
