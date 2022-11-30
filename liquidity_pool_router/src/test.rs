#![cfg(test)]
extern crate std;

use crate::pool_contract::LiquidityPoolClient;
use crate::token::{self, TokenMetadata};

use crate::testutils::{
    register_test_contract as register_liquidity_pool_router, LiquidityPoolRouter,
};
use soroban_sdk::{testutils::Accounts, AccountId, BytesN, Env, IntoVal};
use token::{Identifier, Signature};

fn create_token_contract(e: &Env, admin: &AccountId) -> token::Client {
    let token = token::Client::new(e, e.register_contract_token());

    // decimals, name, symbol don't matter in tests
    token.init(
        &Identifier::Account(admin.clone()),
        &TokenMetadata {
            name: "name".into_val(e),
            symbol: "symbol".into_val(e),
            decimals: 7,
        },
    );
    token
}

fn create_liquidity_pool_router_contract(e: &Env) -> LiquidityPoolRouter {
    LiquidityPoolRouter::new(e, &register_liquidity_pool_router(e))
}

fn install_liquidity_pool_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_liquidity_pool_contract.wasm"
    );
    e.install_contract_wasm(WASM)
}

#[test]
fn test() {
    let e: Env = Default::default();

    let mut admin1 = e.accounts().generate();
    let mut admin2 = e.accounts().generate();

    let mut token1 = create_token_contract(&e, &admin1);
    let mut token2 = create_token_contract(&e, &admin2);
    if &token2.contract_id < &token1.contract_id {
        std::mem::swap(&mut token1, &mut token2);
        std::mem::swap(&mut admin1, &mut admin2);
    }
    let user1 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());

    let contract_router = create_liquidity_pool_router_contract(&e);
    let router_id = Identifier::Contract(contract_router.contract_id.clone());

    token1
        .with_source_account(&admin1)
        .mint(&Signature::Invoker, &0, &user1_id, &1000);
    assert_eq!(token1.balance(&user1_id), 1000);
    token2
        .with_source_account(&admin2)
        .mint(&Signature::Invoker, &0, &user1_id, &1000);
    assert_eq!(token2.balance(&user1_id), 1000);

    token1
        .with_source_account(&user1)
        .approve(&Signature::Invoker, &0, &router_id, &100);
    token2
        .with_source_account(&user1)
        .approve(&Signature::Invoker, &0, &router_id, &100);

    contract_router.sf_deposit(
        &install_liquidity_pool_wasm(&e),
        &user1,
        &token1.contract_id,
        &token2.contract_id,
        &100,
        &100,
        &100,
        &100,
    );

    let contract_pool = contract_router.get_pool(&token1.contract_id, &token2.contract_id);
    let pool_id = Identifier::Contract(contract_pool.clone());

    let share_id = LiquidityPoolClient::new(&e, &contract_pool).share_id();
    let token_share = token::Client::new(&e, &share_id);

    assert_eq!(token1.balance(&user1_id), 900);
    assert_eq!(token1.balance(&pool_id), 100);
    assert_eq!(token2.balance(&user1_id), 900);
    assert_eq!(token2.balance(&pool_id), 100);
    assert_eq!(token_share.balance(&user1_id), 100);
    assert_eq!(token_share.balance(&pool_id), 0);

    token1
        .with_source_account(&user1)
        .approve(&Signature::Invoker, &0, &router_id, &100);

    contract_router.swap_out(&user1, &token1.contract_id, &token2.contract_id, &49, &100);

    assert_eq!(token1.balance(&user1_id), 803);
    assert_eq!(token1.balance(&pool_id), 197);
    assert_eq!(token2.balance(&user1_id), 949);
    assert_eq!(token2.balance(&pool_id), 51);

    token_share
        .with_source_account(&user1)
        .approve(&Signature::Invoker, &0, &router_id, &100);
    contract_router.sf_withdrw(
        &user1,
        &token1.contract_id,
        &token2.contract_id,
        &100,
        &197,
        &51,
    );
    assert_eq!(token1.balance(&user1_id), 1000);
    assert_eq!(token2.balance(&user1_id), 1000);
    assert_eq!(token_share.balance(&user1_id), 0);
    assert_eq!(token1.balance(&pool_id), 0);
    assert_eq!(token2.balance(&pool_id), 0);
    assert_eq!(token_share.balance(&pool_id), 0);
}
