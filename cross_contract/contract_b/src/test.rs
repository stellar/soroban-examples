#![cfg(test)]

use crate::{contract_a, ContractB, ContractBClient};
use soroban_sdk::{BytesN, Env};

#[test]
fn test() {
    let env = Env::default();

    // Define IDs for contract A and B.
    let contract_a_id = BytesN::from_array(&env, &[0; 32]);
    let contract_b_id = BytesN::from_array(&env, &[1; 32]);

    // Register contract A using the imported WASM.
    env.register_contract_wasm(&contract_a_id, contract_a::WASM);

    // Register contract B defined in this crate.
    env.register_contract(&contract_b_id, ContractB);

    // Create a client for calling contract B.
    let client = ContractBClient::new(&env, &contract_b_id);

    // Invoke contract B via its client. Contract B will invoke contract A.
    let sum = client.add_with(&contract_a_id, &5, &7);
    assert_eq!(sum, 12);
}
