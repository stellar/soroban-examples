#![cfg(test)]

use crate::testutils::{register_test_contract as register_single_offer_router, SingleOfferRouter};
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

fn create_single_offer_router_contract(
    e: &Env,
    offer_wasm_hash: &BytesN<32>,
    admin: &AccountId,
    token_a: &BytesN<32>,
    token_b: &BytesN<32>,
    n: u32,
    d: u32,
) -> SingleOfferRouter {
    let single_offer = SingleOfferRouter::new(e, &register_single_offer_router(e));
    single_offer.init(
        offer_wasm_hash,
        &Identifier::Account(admin.clone()),
        token_a,
        token_b,
        n,
        d,
    );
    single_offer
}

fn install_offer_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_single_offer_contract.wasm"
    );
    e.install_contract_wasm(WASM)
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
    // let offer_wasm_hash =
    // The price here is 1 A == .5 B
    let offer_router = create_single_offer_router_contract(
        &e,
        &install_offer_wasm(&e),
        &user1,
        &token1.contract_id,
        &token2.contract_id,
        1,
        1,
    );

    let router_id = Identifier::Contract(offer_router.contract_id.clone());

    // mint tokens that will be traded
    e.set_source_account(&admin1);
    token1.mint(&Signature::Invoker, &0, &user1_id, &20);
    assert_eq!(token1.balance(&user1_id), 20);

    e.set_source_account(&admin2);
    token2.mint(&Signature::Invoker, &0, &user2_id, &20);
    assert_eq!(token2.balance(&user2_id), 20);

    let offer_addr = offer_router.get_offer(&user1_id, &token1.contract_id, &token2.contract_id);
    let offer_id = Identifier::Contract(offer_addr.clone());

    // admin transfers the sell_token (token1) to the contract address
    token1
        .with_source_account(&user1)
        .xfer(&Signature::Invoker, &0, &offer_id, &10);

    // set required allowances for the router contract before trading
    token2
        .with_source_account(&user2)
        .incr_allow(&Signature::Invoker, &0, &router_id, &20);

    offer_router.safe_trade(&user2, &offer_addr, &10, &10);
    assert_eq!(token1.balance(&user1_id), 10);
    assert_eq!(token1.balance(&user2_id), 10);
    assert_eq!(token2.balance(&user1_id), 10);
    assert_eq!(token2.balance(&user2_id), 10);
}
