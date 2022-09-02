#![cfg(test)]

use crate::testutils::{register_test_contract as register_single_offer, SingleOfferXferFrom};
use ed25519_dalek::Keypair;
use rand::{thread_rng, RngCore};
use soroban_auth::Identifier;
use soroban_sdk::{BigInt, BytesN, Env};
use soroban_token_contract::testutils::{
    register_test_contract as register_token, to_ed25519, Token,
};

fn generate_contract_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn create_token_contract(e: &Env, admin: &Keypair) -> ([u8; 32], Token) {
    let id = generate_contract_id();
    register_token(&e, &id);
    let token = Token::new(e, &id);
    // decimals, name, symbol don't matter in tests
    token.initialize(&to_ed25519(&e, admin), 7, "name", "symbol");
    (id, token)
}

fn create_single_offer_contract(
    e: &Env,
    admin: &Keypair,
    token_a: &[u8; 32],
    token_b: &[u8; 32],
    n: u32,
    d: u32,
) -> ([u8; 32], SingleOfferXferFrom) {
    let id = generate_contract_id();
    register_single_offer(&e, &id);
    let single_offer = SingleOfferXferFrom::new(e, &id);
    single_offer.initialize(&to_ed25519(&e, admin), token_a, token_b, n, d);
    (id, single_offer)
}

#[test]
fn test() {
    let e: Env = Default::default();

    let admin1 = generate_keypair();
    let admin2 = generate_keypair();
    let user1 = generate_keypair();
    let user1_id = to_ed25519(&e, &user1);
    let user2 = generate_keypair();
    let user2_id = to_ed25519(&e, &user2);

    let (contract1, token1) = create_token_contract(&e, &admin1);
    let (contract2, token2) = create_token_contract(&e, &admin2);

    // The price here is 1 A == .5 B and the admin in user1
    let (contract_offer, offer) =
        create_single_offer_contract(&e, &user1, &contract1, &contract2, 1, 2);
    let offer_id = Identifier::Contract(BytesN::from_array(&e, &contract_offer));

    // mint tokens that will be traded
    token1.mint(&admin1, &user1_id, &BigInt::from_u32(&e, 30));
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 30));
    token2.mint(&admin2, &user2_id, &BigInt::from_u32(&e, 20));
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 20));

    // set required allowances before trading
    token1.approve(&user1, &offer_id, &BigInt::from_u32(&e, 30));
    token2.approve(&user2, &offer_id, &BigInt::from_u32(&e, 20));

    offer.trade(&user2, &BigInt::from_u32(&e, 10), &BigInt::from_u32(&e, 20));
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 10));
    assert_eq!(token1.balance(&user2_id), BigInt::from_u32(&e, 20));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 10));
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 10));

    // The price here is 1 A = 1 B
    offer.updt_price(&user1, 1, 1);

    // Trade 10 token2 for 10 token1
    offer.trade(&user2, &BigInt::from_u32(&e, 10), &BigInt::from_u32(&e, 10));
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 0));
    assert_eq!(token1.balance(&user2_id), BigInt::from_u32(&e, 30));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 20));
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 0));
}
