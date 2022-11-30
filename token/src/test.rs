#![cfg(test)]

use crate::testutils::{register_test_contract as register_token, to_ed25519, Token};
use crate::TokenClient;
use ed25519_dalek::Keypair;
use rand::{thread_rng, RngCore};
use soroban_auth::{Ed25519Signature, Signature};
use soroban_sdk::{Env, IntoVal};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

#[test]
fn test() {
    let e: Env = Default::default();
    let contract_id = register_token(&e);
    let token = Token::new(&e, &contract_id);

    let admin1 = generate_keypair();
    let admin1_id = to_ed25519(&e, &admin1);
    let admin2 = generate_keypair();
    let admin2_id = to_ed25519(&e, &admin2);
    let user1 = generate_keypair();
    let user1_id = to_ed25519(&e, &user1);
    let user2 = generate_keypair();
    let user2_id = to_ed25519(&e, &user2);
    let user3 = generate_keypair();
    let user3_id = to_ed25519(&e, &user3);

    token.initialize(&admin1_id, 7, "name", "symbol");

    token.mint(&admin1, &user1_id, &1000);
    assert_eq!(token.balance(&user1_id), 1000);
    assert_eq!(token.nonce(&admin1_id), 1);

    token.approve(&user2, &user3_id, &500);
    assert_eq!(token.allowance(&user2_id, &user3_id), 500);
    assert_eq!(token.nonce(&user2_id), 1);

    token.xfer(&user1, &user2_id, &600);
    assert_eq!(token.balance(&user1_id), 400);
    assert_eq!(token.balance(&user2_id), 600);
    assert_eq!(token.nonce(&user1_id), 1);

    token.xfer_from(&user3, &user2_id, &user1_id, &400);
    assert_eq!(token.balance(&user1_id), 800);
    assert_eq!(token.balance(&user2_id), 200);
    assert_eq!(token.nonce(&user3_id), 1);

    token.xfer(&user1, &user3_id, &300);
    assert_eq!(token.balance(&user1_id), 500);
    assert_eq!(token.balance(&user3_id), 300);
    assert_eq!(token.nonce(&user1_id), 2);

    token.set_admin(&admin1, &admin2_id);
    assert_eq!(token.nonce(&admin1_id), 2);

    token.freeze(&admin2, &user2_id);
    assert_eq!(token.is_frozen(&user2_id), true);
    assert_eq!(token.nonce(&admin2_id), 1);

    token.unfreeze(&admin2, &user3_id);
    assert_eq!(token.is_frozen(&user3_id), false);
    assert_eq!(token.nonce(&admin2_id), 2);

    token.burn(&admin2, &user3_id, &100);
    assert_eq!(token.balance(&user3_id), 200);
    assert_eq!(token.nonce(&admin2_id), 3);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn xfer_insufficient_balance() {
    let e: Env = Default::default();
    let contract_id = register_token(&e);
    let token = Token::new(&e, &contract_id);

    let admin1 = generate_keypair();
    let user1 = generate_keypair();
    let user2 = generate_keypair();
    let admin1_id = to_ed25519(&e, &admin1);
    let user1_id = to_ed25519(&e, &user1);
    let user2_id = to_ed25519(&e, &user2);

    token.initialize(&admin1_id, 10, "name", "symbol");

    token.mint(&admin1, &user1_id, &1000);
    assert_eq!(token.balance(&user1_id), 1000);
    assert_eq!(token.nonce(&admin1_id), 1);

    token.xfer(&user1, &user2_id, &1001);
}

#[test]
#[should_panic(expected = "can't receive when frozen")]
fn xfer_receive_frozen() {
    let e: Env = Default::default();
    let contract_id = register_token(&e);
    let token = Token::new(&e, &contract_id);

    let admin1 = generate_keypair();
    let user1 = generate_keypair();
    let user2 = generate_keypair();
    let admin1_id = to_ed25519(&e, &admin1);
    let user1_id = to_ed25519(&e, &user1);
    let user2_id = to_ed25519(&e, &user2);

    token.initialize(&admin1_id, 10, "name", "symbol");

    token.mint(&admin1, &user1_id, &1000);
    assert_eq!(token.balance(&user1_id), 1000);
    assert_eq!(token.nonce(&admin1_id), 1);

    token.freeze(&admin1, &user2_id);
    token.xfer(&user1, &user2_id, &1);
}

#[test]
#[should_panic(expected = "can't spend when frozen")]
fn xfer_spend_frozen() {
    let e: Env = Default::default();
    let contract_id = register_token(&e);
    let token = Token::new(&e, &contract_id);

    let admin1 = generate_keypair();
    let user1 = generate_keypair();
    let user2 = generate_keypair();
    let admin1_id = to_ed25519(&e, &admin1);
    let user1_id = to_ed25519(&e, &user1);
    let user2_id = to_ed25519(&e, &user2);

    token.initialize(&admin1_id, 10, "name", "symbol");

    token.mint(&admin1, &user1_id, &1000);
    assert_eq!(token.balance(&user1_id), 1000);
    assert_eq!(token.nonce(&admin1_id), 1);

    token.freeze(&admin1, &user1_id);
    token.xfer(&user1, &user2_id, &1);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn xfer_from_insufficient_allowance() {
    let e: Env = Default::default();
    let contract_id = register_token(&e);
    let token = Token::new(&e, &contract_id);

    let admin1 = generate_keypair();
    let user1 = generate_keypair();
    let user2 = generate_keypair();
    let user3 = generate_keypair();
    let admin1_id = to_ed25519(&e, &admin1);
    let user1_id = to_ed25519(&e, &user1);
    let user2_id = to_ed25519(&e, &user2);
    let user3_id = to_ed25519(&e, &user3);

    token.initialize(&admin1_id, 10, "name", "symbol");

    token.mint(&admin1, &user1_id, &1000);
    assert_eq!(token.balance(&user1_id), 1000);
    assert_eq!(token.nonce(&admin1_id), 1);

    token.approve(&user1, &user3_id, &100);
    assert_eq!(token.allowance(&user1_id, &user3_id), 100);
    assert_eq!(token.nonce(&user1_id), 1);

    token.xfer_from(&user3, &user1_id, &user2_id, &101);
}

#[test]
#[should_panic(expected = "already initialized")]
fn initialize_already_initialized() {
    let e: Env = Default::default();
    let contract_id = register_token(&e);
    let token = Token::new(&e, &contract_id);

    let admin1 = generate_keypair();
    let admin1_id = to_ed25519(&e, &admin1);

    token.initialize(&admin1_id, 10, "name", "symbol");
    token.initialize(&admin1_id, 10, "name", "symbol");
}

#[test]
#[should_panic] // TODO: Add expected
fn set_admin_bad_signature() {
    let e: Env = Default::default();
    let contract_id = register_token(&e);
    let token = Token::new(&e, &contract_id);

    let admin1 = generate_keypair();
    let admin2 = generate_keypair();
    let admin1_id = to_ed25519(&e, &admin1);
    let admin2_id = to_ed25519(&e, &admin2);

    token.initialize(&admin1_id, 10, "name", "symbol");

    let mut signature = [0u8; 64];
    thread_rng().fill_bytes(&mut signature);

    let auth = Signature::Ed25519(Ed25519Signature {
        public_key: admin1.public.to_bytes().into_val(&e),
        signature: signature.into_val(&e),
    });

    let client = TokenClient::new(&e, &contract_id);
    let nonce = client.nonce(&admin1_id);
    client.set_admin(&auth, &nonce, &admin2_id);
}

#[test]
#[should_panic(expected = "Decimal must fit in a u8")]
fn decimal_is_over_max() {
    let e = Default::default();
    let contract_id = register_token(&e);
    let token = Token::new(&e, &contract_id);

    let admin1 = generate_keypair();
    let admin1_id = to_ed25519(&e, &admin1);

    token.initialize(&admin1_id, u32::from(u8::MAX) + 1, "name", "symbol");
}
