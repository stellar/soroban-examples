#![cfg(test)]

use super::*;
use soroban_sdk::{symbol, vec, BytesN, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol!("Dev"));
    assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
}
