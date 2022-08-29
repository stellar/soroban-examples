#![cfg(test)]

use crate::b::ContractBClient;

use super::{a::ContractA, b::ContractB};
use soroban_sdk::{BytesN, Env};

#[test]
fn test() {
    let env = Env::default();

    let contract_a = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_a, ContractA);

    let contract_b = BytesN::from_array(&env, &[1; 32]);
    env.register_contract(&contract_b, ContractB);

    // Invoke 'add_with' on contract B.
    let sum = ContractBClient::new(&env, &contract_b).add_with(&5, &7, &contract_a);
    assert_eq!(sum, 12);
}
