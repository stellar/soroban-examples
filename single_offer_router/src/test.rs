#![cfg(test)]

use crate::testutils::{register_test_contract as register_single_offer_router, SingleOfferRouter};
use crate::token::{self, TokenMetadata};
use rand::{thread_rng, RngCore};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{testutils::Accounts, AccountId, BigInt, BytesN, Env, IntoVal};

fn generate_contract_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}

fn create_token_contract(e: &Env, admin: &AccountId) -> ([u8; 32], token::Client) {
    let id = e.register_contract_token(None);
    let token = token::Client::new(e, &id);
    // decimals, name, symbol don't matter in tests
    token.init(
        &Identifier::Account(admin.clone()),
        &TokenMetadata {
            name: "name".into_val(e),
            symbol: "symbol".into_val(e),
            decimals: 7,
        },
    );
    (id.into(), token)
}

fn create_single_offer_router_contract(
    e: &Env,
    admin: &AccountId,
    token_a: &[u8; 32],
    token_b: &[u8; 32],
    n: u32,
    d: u32,
) -> ([u8; 32], SingleOfferRouter) {
    let id = generate_contract_id();
    register_single_offer_router(&e, &id);
    let single_offer = SingleOfferRouter::new(e, &id);
    single_offer.init(&Identifier::Account(admin.clone()), token_a, token_b, n, d);
    (id, single_offer)
}

#[test]
fn test() {
    let e: Env = Default::default();

    let admin1 = e.accounts().generate();
    let admin2 = e.accounts().generate();
    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());

    let (contract1, token1) = create_token_contract(&e, &admin1);
    let (contract2, token2) = create_token_contract(&e, &admin2);
    // The price here is 1 A == .5 B
    let (contract_offer_router, offer_router) =
        create_single_offer_router_contract(&e, &user1, &contract1, &contract2, 1, 1);

    let router_id = Identifier::Contract(BytesN::from_array(&e, &contract_offer_router));

    // mint tokens that will be traded
    e.set_source_account(&admin1);
    token1.mint(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &user1_id,
        &BigInt::from_u32(&e, 20),
    );
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 20));

    e.set_source_account(&admin2);
    token2.mint(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &user2_id,
        &BigInt::from_u32(&e, 20),
    );
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 20));

    let offer_addr = offer_router.get_offer(&user1_id, &contract1, &contract2);
    let offer_id = Identifier::Contract(offer_addr.clone());

    // admin transfers the sell_token (token1) to the contract address
    token1.with_source_account(&user1).xfer(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &offer_id,
        &BigInt::from_u32(&e, 10),
    );

    // set required allowances for the router contract before trading
    token2.with_source_account(&user2).approve(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &router_id,
        &BigInt::from_u32(&e, 20),
    );

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
