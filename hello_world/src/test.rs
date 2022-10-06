#![cfg(test)]

use super::*;
use soroban_sdk::{symbol, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol!("Dev"));
    assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
}
