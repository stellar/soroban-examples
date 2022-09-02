#![cfg(test)]

use crate::testutils::{register_test_contract as register_single_offer_router, SingleOfferRouter};
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

fn create_single_offer_router_contract(
    e: &Env,
    admin: &Keypair,
    token_a: &[u8; 32],
    token_b: &[u8; 32],
    n: u32,
    d: u32,
) -> ([u8; 32], SingleOfferRouter) {
    let id = generate_contract_id();
    register_single_offer_router(&e, &id);
    let single_offer = SingleOfferRouter::new(e, &id);
    single_offer.init(&to_ed25519(&e, admin), token_a, token_b, n, d);
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
    // The price here is 1 A == .5 B
    let (contract_offer_router, offer_router) =
        create_single_offer_router_contract(&e, &user1, &contract1, &contract2, 1, 1);

    let router_id = Identifier::Contract(BytesN::from_array(&e, &contract_offer_router));

    // mint tokens that will be traded
    token1.mint(&admin1, &user1_id, &BigInt::from_u32(&e, 20));
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 20));
    token2.mint(&admin2, &user2_id, &BigInt::from_u32(&e, 20));
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 20));

    let offer_addr = offer_router.get_offer(&user1_id, &contract1, &contract2);
    let offer_id = Identifier::Contract(offer_addr.clone());

    // admin transfers the sell_token (token1) to the contract address
    token1.xfer(&user1, &offer_id, &BigInt::from_u32(&e, 10));

    // set required allowances for the router contract before trading
    token2.approve(&user2, &router_id, &BigInt::from_u32(&e, 20));

    offer_router.safe_trade(
        &user2,
        &offer_addr.into(),
        &BigInt::from_u32(&e, 10),
        &BigInt::from_u32(&e, 10),
    );
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 10));
    assert_eq!(token1.balance(&user2_id), BigInt::from_u32(&e, 10));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 10));
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 10));
}
