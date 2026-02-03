#![cfg(test)]

use super::*;
use soroban_sdk::{Env, U256};

#[test]
fn test_store_and_load_u32() {
    let env = Env::default();
    let contract_id = env.register(TokenIdContract, ());
    let client = TokenIdContractClient::new(&env, &contract_id);

    client.store_u32(&42);
    assert_eq!(client.load_u32(), 42);
}

#[test]
fn test_event_u32() {
    let env = Env::default();
    let contract_id = env.register(TokenIdContract, ());
    let client = TokenIdContractClient::new(&env, &contract_id);

    client.event_u32(&123);
    // Event was published (no assertion needed, just verify it doesn't panic)
}

#[test]
fn test_store_and_load_u256() {
    let env = Env::default();
    let contract_id = env.register(TokenIdContract, ());
    let client = TokenIdContractClient::new(&env, &contract_id);

    let token_id = U256::from_u32(&env, 999);
    client.store_u256(&token_id);
    assert_eq!(client.load_u256(), token_id);
}

#[test]
fn test_event_u256() {
    let env = Env::default();
    let contract_id = env.register(TokenIdContract, ());
    let client = TokenIdContractClient::new(&env, &contract_id);

    let token_id = U256::from_u32(&env, 456);
    client.event_u256(&token_id);
    // Event was published (no assertion needed, just verify it doesn't panic)
}
