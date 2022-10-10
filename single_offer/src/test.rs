#![cfg(test)]

use crate::testutils::{register_test_contract as register_single_offer, SingleOffer};
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

fn create_single_offer_contract(
    e: &Env,
    admin: &AccountId,
    token_a: &[u8; 32],
    token_b: &[u8; 32],
    n: u32,
    d: u32,
) -> ([u8; 32], SingleOffer) {
    let id = generate_contract_id();
    register_single_offer(&e, &id);
    let single_offer = SingleOffer::new(e, &id);
    single_offer.initialize(&Identifier::Account(admin.clone()), token_a, token_b, n, d);
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
    let (contract_offer, offer) =
        create_single_offer_contract(&e, &user1, &contract1, &contract2, 1, 2);
    let offer_id = Identifier::Contract(BytesN::from_array(&e, &contract_offer));

    token1.with_source_account(&admin1).mint(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &user1_id,
        &BigInt::from_u32(&e, 1000),
    );

    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 1000));
    token2.with_source_account(&admin2).mint(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &user2_id,
        &BigInt::from_u32(&e, 1000),
    );
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 1000));

    // Deposit 100 token1 into offer
    token1.with_source_account(&user1).xfer(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &offer_id,
        &BigInt::from_u32(&e, 100),
    );
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 900));
    assert_eq!(token1.balance(&offer_id), BigInt::from_u32(&e, 100));

    // Trade 10 token2 for 20 token1
    token2.with_source_account(&user2).xfer(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &offer_id,
        &BigInt::from_u32(&e, 10),
    );
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 990));
    assert_eq!(token2.balance(&offer_id), BigInt::from_u32(&e, 10));
    offer.trade(&user2_id, &BigInt::from_u32(&e, 20));
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 900));
    assert_eq!(token1.balance(&user2_id), BigInt::from_u32(&e, 20));
    assert_eq!(token1.balance(&offer_id), BigInt::from_u32(&e, 80));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 10));
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 990));
    assert_eq!(token2.balance(&offer_id), BigInt::zero(&e));

    // Withdraw 70 token1 from offer
    offer.withdraw(&user1, &BigInt::from_u32(&e, 70));
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 970));
    assert_eq!(token1.balance(&user2_id), BigInt::from_u32(&e, 20));
    assert_eq!(token1.balance(&offer_id), BigInt::from_u32(&e, 10));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 10));
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 990));
    assert_eq!(token2.balance(&offer_id), BigInt::zero(&e));

    // The price here is 1 A = 1 B
    offer.updt_price(&user1, 1, 1);

    // Trade 10 token2 for 10 token1
    token2.with_source_account(&user2).xfer(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &offer_id,
        &BigInt::from_u32(&e, 10),
    );
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 980));
    assert_eq!(token2.balance(&offer_id), BigInt::from_u32(&e, 10));
    offer.trade(&user2_id, &BigInt::from_u32(&e, 10));
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 970));
    assert_eq!(token1.balance(&user2_id), BigInt::from_u32(&e, 30));
    assert_eq!(token1.balance(&offer_id), BigInt::from_u32(&e, 00));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 20));
    assert_eq!(token2.balance(&user2_id), BigInt::from_u32(&e, 980));
    assert_eq!(token2.balance(&offer_id), BigInt::zero(&e));
}
