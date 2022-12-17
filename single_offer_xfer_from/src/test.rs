#![cfg(test)]

use crate::testutils::{register_test_contract as register_single_offer, SingleOfferXferFrom};
use soroban_sdk::{testutils::Accounts, AccountId, BytesN, Env, IntoVal};

soroban_sdk::contractimport!(
    file = "../target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
);

type TokenClient = Client;

fn create_token_contract(e: &Env, admin: &AccountId) -> TokenClient {
    e.install_contract_wasm(WASM);

    let token = TokenClient::new(e, e.register_contract_wasm(None, WASM));
    // decimals, name, symbol don't matter in tests
    token.initialize(
        &Identifier::Account(admin.clone()),
        &7u32,
        &"name".into_val(e),
        &"symbol".into_val(e),
    );
    token
}

fn create_single_offer_contract(
    e: &Env,
    admin: &AccountId,
    token_a: &BytesN<32>,
    token_b: &BytesN<32>,
    n: u32,
    d: u32,
) -> SingleOfferXferFrom {
    let single_offer = SingleOfferXferFrom::new(e, &register_single_offer(e));
    single_offer.initialize(&Identifier::Account(admin.clone()), token_a, token_b, n, d);
    single_offer
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

    let token1 = create_token_contract(&e, &admin1);
    let token2 = create_token_contract(&e, &admin2);

    // The price here is 1 A == .5 B and the admin in user1
    let offer =
        create_single_offer_contract(&e, &user1, &token1.contract_id, &token2.contract_id, 1, 2);
    let offer_id = Identifier::Contract(offer.contract_id.clone());

    // mint tokens that will be traded
    token1
        .with_source_account(&admin1)
        .mint(&Signature::Invoker, &0, &user1_id, &30);
    assert_eq!(token1.balance(&user1_id), 30);
    token2
        .with_source_account(&admin2)
        .mint(&Signature::Invoker, &0, &user2_id, &20);
    assert_eq!(token2.balance(&user2_id), 20);

    // set required allowances before trading
    token1
        .with_source_account(&user1)
        .approve(&Signature::Invoker, &0, &offer_id, &30);
    token2
        .with_source_account(&user2)
        .approve(&Signature::Invoker, &0, &offer_id, &20);

    offer.trade(&user2, &10, &20);
    assert_eq!(token1.balance(&user1_id), 10);
    assert_eq!(token1.balance(&user2_id), 20);
    assert_eq!(token2.balance(&user1_id), 10);
    assert_eq!(token2.balance(&user2_id), 10);

    // The price here is 1 A = 1 B
    offer.updt_price(&user1, 1, 1);

    // Trade 10 token2 for 10 token1
    offer.trade(&user2, &10, &10);
    assert_eq!(token1.balance(&user1_id), 0);
    assert_eq!(token1.balance(&user2_id), 30);
    assert_eq!(token2.balance(&user1_id), 20);
    assert_eq!(token2.balance(&user2_id), 0);
}
