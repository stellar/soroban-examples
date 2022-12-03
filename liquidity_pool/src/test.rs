#![cfg(test)]
extern crate std;

use crate::testutils::{register_test_contract as register_liqpool, LiquidityPool};
use crate::token::{self, TokenMetadata};
use soroban_sdk::{testutils::Accounts, AccountId, BytesN, Env, IntoVal};
use token::{Identifier, Signature};

fn create_token_contract(e: &Env, admin: &AccountId) -> token::Client {
    let token = token::Client::new(e, e.register_contract_token(None));
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

fn create_liqpool_contract(e: &Env, token_a: &BytesN<32>, token_b: &BytesN<32>) -> LiquidityPool {
    let liqpool = LiquidityPool::new(e, &register_liqpool(e));
    liqpool.initialize(token_a, token_b);
    liqpool
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
    let liqpool = create_liqpool_contract(&e, &token1.contract_id, &token2.contract_id);
    let pool_id = Identifier::Contract(liqpool.contract_id.clone());
    let contract_share: [u8; 32] = liqpool.share_id().into();
    let token_share = token::Client::new(&e, contract_share);

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
        .xfer(&Signature::Invoker, &0, &pool_id, &100);
    assert_eq!(token1.balance(&user1_id), 900);
    assert_eq!(token1.balance(&pool_id), 100);

    token2
        .with_source_account(&user1)
        .xfer(&Signature::Invoker, &0, &pool_id, &100);
    assert_eq!(token2.balance(&user1_id), 900);
    assert_eq!(token2.balance(&pool_id), 100);
    liqpool.deposit(&user1_id);
    assert_eq!(token_share.balance(&user1_id), 100);
    assert_eq!(token_share.balance(&pool_id), 0);

    token1
        .with_source_account(&user1)
        .xfer(&Signature::Invoker, &0, &pool_id, &100);
    assert_eq!(token1.balance(&user1_id), 800);
    assert_eq!(token1.balance(&pool_id), 200);
    liqpool.swap(&user1_id, &0, &49);
    assert_eq!(token1.balance(&user1_id), 800);
    assert_eq!(token1.balance(&pool_id), 200);
    assert_eq!(token2.balance(&user1_id), 949);
    assert_eq!(token2.balance(&pool_id), 51);

    token_share
        .with_source_account(&user1)
        .xfer(&Signature::Invoker, &0, &pool_id, &100);
    assert_eq!(token_share.balance(&user1_id), 0);
    assert_eq!(token_share.balance(&pool_id), 100);
    liqpool.withdraw(&user1_id);
    assert_eq!(token1.balance(&user1_id), 1000);
    assert_eq!(token2.balance(&user1_id), 1000);
    assert_eq!(token_share.balance(&user1_id), 0);
    assert_eq!(token1.balance(&pool_id), 0);
    assert_eq!(token2.balance(&pool_id), 0);
    assert_eq!(token_share.balance(&pool_id), 0);
}
