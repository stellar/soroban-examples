#![cfg(test)]

use crate::testutils::{
    register_test_contract as register_liquidity_pool_router, LiquidityPoolRouter,
};
use ed25519_dalek::Keypair;
use liquidity_pool::LiquidityPoolClient;
use rand::{thread_rng, RngCore};
use soroban_liquidity_pool_contract as liquidity_pool;
use soroban_sdk::{BigInt, BytesN, Env};
use soroban_sdk_auth::public_types::Identifier;
use soroban_token_contract::testutils::{
    register_test_contract as register_token, to_ed25519, Token,
};

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

fn create_token_contract(e: &Env, id: &[u8; 32], admin: &Keypair) -> Token {
    register_token(&e, id);
    let token = Token::new(e, id);
    // decimals, name, symbol don't matter in tests
    token.initialize(&to_ed25519(&e, admin), 7, "name", "symbol");
    token
}

fn generate_contract_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
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

    let admin1 = generate_keypair();
    let admin2 = generate_keypair();
    let user1 = generate_keypair();
    let user1_id = to_ed25519(&e, &user1);

    let (contract1, contract2) = generate_sorted_contract_ids();
    let token1 = create_token_contract(&e, &contract1, &admin1);
    let token2 = create_token_contract(&e, &contract2, &admin2);
    let (contract_router, router) = create_liquidity_pool_router_contract(&e);
    let router_id = Identifier::Contract(BytesN::from_array(&e, &contract_router));

    token1.mint(&admin1, &user1_id, &BigInt::from_u32(&e, 1000));
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 1000));
    token2.mint(&admin2, &user1_id, &BigInt::from_u32(&e, 1000));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 1000));

    token1.approve(&user1, &router_id, &BigInt::from_u32(&e, 100));
    token2.approve(&user1, &router_id, &BigInt::from_u32(&e, 100));

    router.sf_deposit(
        &user1_id,
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
    let token_share = Token::new(&e, &share_id.into());

    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 900));
    assert_eq!(token1.balance(&pool_id), BigInt::from_u32(&e, 100));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 900));
    assert_eq!(token2.balance(&pool_id), BigInt::from_u32(&e, 100));
    assert_eq!(token_share.balance(&user1_id), BigInt::from_u32(&e, 100));
    assert_eq!(token_share.balance(&pool_id), BigInt::from_u32(&e, 0));

    token1.approve(&user1, &router_id, &BigInt::from_u32(&e, 100));

    router.swap_out(
        &user1_id,
        &contract1,
        &contract2,
        &BigInt::from_u32(&e, 49),
        &BigInt::from_u32(&e, 100),
    );

    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 803));
    assert_eq!(token1.balance(&pool_id), BigInt::from_u32(&e, 197));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 949));
    assert_eq!(token2.balance(&pool_id), BigInt::from_u32(&e, 51));

    token_share.approve(&user1, &router_id, &BigInt::from_u32(&e, 100));
    router.sf_withdrw(
        &user1_id,
        &contract1,
        &contract2,
        &BigInt::from_u32(&e, 100),
        &BigInt::from_u32(&e, 197),
        &BigInt::from_u32(&e, 51),
    );
    assert_eq!(token1.balance(&user1_id), BigInt::from_u32(&e, 1000));
    assert_eq!(token2.balance(&user1_id), BigInt::from_u32(&e, 1000));
    assert_eq!(token_share.balance(&user1_id), BigInt::from_u32(&e, 0));
    assert_eq!(token1.balance(&pool_id), BigInt::from_u32(&e, 0));
    assert_eq!(token2.balance(&pool_id), BigInt::from_u32(&e, 0));
    assert_eq!(token_share.balance(&pool_id), BigInt::from_u32(&e, 0));
}
