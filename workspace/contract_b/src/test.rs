#![cfg(test)]

use soroban_sdk::Env;

use crate::{ContractB, ContractBClient};

use soroban_workspace_contract_a::ContractA;

#[test]
fn test() {
    let env = Env::default();

    // Register contract A using the native contract imported.
    let contract_a_id = env.register_contract(None, ContractA);

    // Register contract B defined in this crate.
    let contract_b_id = env.register_contract(None, ContractB);

    // Create a client for calling contract B.
    let client = ContractBClient::new(&env, &contract_b_id);

    // Invoke contract B via its client. Contract B will invoke contract A.
    let sum = client.add_with(&contract_a_id, &5, &7);
    assert_eq!(sum, 12);
}
