#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::Env;

extern crate std;

#[test]
fn test_passes() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    for i in 0..2207 {
        assert_eq!(client.increment(), i + 1);
    }
}

#[test]
fn test_stack_overflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    for i in 0..2208 {
        assert_eq!(client.increment(), i + 1);
    }
}
