#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Accounts, BytesN, Env};

#[test]
fn test_auth_with_invoker() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    assert_eq!(
        client
            .with_source_account(&user_1)
            .increment(&Signature::Invoker, &0),
        1
    );
    assert_eq!(
        client
            .with_source_account(&user_1)
            .increment(&Signature::Invoker, &0),
        2
    );
    assert_eq!(
        client
            .with_source_account(&user_2)
            .increment(&Signature::Invoker, &0),
        1
    );
    assert_eq!(
        client
            .with_source_account(&user_1)
            .increment(&Signature::Invoker, &0),
        3
    );
}

#[test]
fn test_auth_with_ed25519() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let (user_1_id, user_1_sign) = soroban_auth::testutils::ed25519::generate(&env);
    let (user_2_id, user_2_sign) = soroban_auth::testutils::ed25519::generate(&env);

    let nonce = 0;
    let sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("increment"),
        (user_1_id.clone(), nonce),
    );
    assert_eq!(client.increment(&sig, &nonce), 1);

    let nonce = 1;
    let sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("increment"),
        (user_1_id.clone(), nonce),
    );
    assert_eq!(client.increment(&sig, &nonce), 2);

    let nonce = 0;
    let sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_2_sign,
        &contract_id,
        symbol!("increment"),
        (user_2_id.clone(), nonce),
    );
    assert_eq!(client.increment(&sig, &nonce), 1);

    let nonce = 2;
    let sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("increment"),
        (user_1_id, nonce),
    );
    assert_eq!(client.increment(&sig, &nonce), 3);
}

#[test]
#[should_panic(expected = "Status(UnknownError(0))")]
fn test_auth_with_ed25519_wrong_signer() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let (user_1_id, _) = soroban_auth::testutils::ed25519::generate(&env);
    let (_, user_2_sign) = soroban_auth::testutils::ed25519::generate(&env);

    // User 2 signs but claims to be user 1.
    let nonce = 0;
    let sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_2_sign,
        &contract_id,
        symbol!("increment"),
        (user_1_id, nonce),
    );
    assert_eq!(client.increment(&sig, &nonce), 1);
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_auth_with_ed25519_wrong_nonce() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let (user_1_id, user_1_sign) = soroban_auth::testutils::ed25519::generate(&env);

    // User 1 signs using incorrect next expected nonce.
    let nonce = 1;
    let sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("increment"),
        (user_1_id, nonce),
    );
    assert_eq!(client.increment(&sig, &nonce), 1);
}
