#![cfg(test)]

use crate::token::{self, TokenMetadata};

use crate::testutils::{
    register_test_contract as register_liquidity_pool_router, LiquidityPoolRouter,
};
use liquidity_pool::LiquidityPoolClient;
use rand::{thread_rng, RngCore};
use soroban_auth::{Identifier, Signature};
use soroban_liquidity_pool_contract as liquidity_pool;
use soroban_sdk::{testutils::Accounts, AccountId, BigInt, BytesN, Env, IntoVal};

fn generate_sorted_contract_ids() -> ([u8; 32], [u8; 32]) {
    let a = generate_contract_id();
    let b = generate_contract_id();
    if a < b {
        (a, b)
    } else if a == b {
        generate_sorted_contract_ids()
    } else {
        (b, a)
    }
}

fn create_token_contract(e: &Env, id: &[u8; 32], admin: &AccountId) -> token::Client {
    let contract_id = BytesN::from_array(e, &id);
    e.register_contract_token(&contract_id);
    let token = token::Client::new(e, id);

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

fn generate_contract_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}

fn create_liquidity_pool_router_contract(e: &Env) -> ([u8; 32], LiquidityPoolRouter) {
    let id = generate_contract_id();
    register_liquidity_pool_router(&e, &id);
    let pool = LiquidityPoolRouter::new(e, &id);
    (id, pool)
}

#[test]
fn test() {
    let e: Env = Default::default();

    let admin1 = e.accounts().generate();
    let admin2 = e.accounts().generate();
    let user1 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());

    let (contract1, contract2) = generate_sorted_contract_ids();
    let token1 = create_token_contract(&e, &contract1, &admin1);
    let token2 = create_token_contract(&e, &contract2, &admin2);
    let (contract_router, router) = create_liquidity_pool_router_contract(&e);
    let router_id = Identifier::Contract(BytesN::from_array(&e, &contract_router));

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
        &user1_id,
        &BigInt::from_u32(&e, 1000),
    );
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 1000));

    token1.with_source_account(&user1).approve(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &router_id,
        &BigInt::from_u32(&e, 100),
    );
    token2.with_source_account(&user1).approve(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &router_id,
        &BigInt::from_u32(&e, 100),
    );

    router.sf_deposit(
        &user1,
        &contract1,
        &contract2,
        &BigInt::from_u32(&e, 100),
        &BigInt::from_u32(&e, 100),
        &BigInt::from_u32(&e, 100),
        &BigInt::from_u32(&e, 100),
    );

    let contract_pool = router.get_pool(&contract1, &contract2);
    let pool_id = Identifier::Contract(contract_pool.clone());

    let share_id = LiquidityPoolClient::new(&e, &contract_pool).share_id();
    let token_share = token::Client::new(&e, &share_id);

    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 900));
    assert_eq!(token1.balance(&pool_id), BigInt::from_u32(&e, 100));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 900));
    assert_eq!(token2.balance(&pool_id), BigInt::from_u32(&e, 100));
    assert_eq!(token_share.balance(&user1_id), BigInt::from_u32(&e, 100));
    assert_eq!(token_share.balance(&pool_id), BigInt::zero(&e));

    token1.with_source_account(&user1).approve(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &router_id,
        &BigInt::from_u32(&e, 100),
    );

    router.swap_out(
        &user1,
        &contract1,
        &contract2,
        &BigInt::from_u32(&e, 49),
        &BigInt::from_u32(&e, 100),
    );

    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 803));
    assert_eq!(token1.balance(&pool_id), BigInt::from_u32(&e, 197));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 949));
    assert_eq!(token2.balance(&pool_id), BigInt::from_u32(&e, 51));

    token_share.with_source_account(&user1).approve(
        &Signature::Invoker,
        &BigInt::zero(&e),
        &router_id,
        &BigInt::from_u32(&e, 100),
    );
    router.sf_withdrw(
        &user1,
        &contract1,
        &contract2,
        &BigInt::from_u32(&e, 100),
        &BigInt::from_u32(&e, 197),
        &BigInt::from_u32(&e, 51),
    );
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 1000));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 1000));
    assert_eq!(token_share.balance(&user1_id), BigInt::zero(&e));
    assert_eq!(token1.balance(&pool_id), BigInt::zero(&e));
    assert_eq!(token2.balance(&pool_id), BigInt::zero(&e));
    assert_eq!(token_share.balance(&pool_id), BigInt::zero(&e));
}
