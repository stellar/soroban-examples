#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CustomTypesContract);
    let client = CustomTypesContractClient::new(&env, &contract_id);
    assert_eq!(client.u32_fail_on_even(&1), 1);
}
