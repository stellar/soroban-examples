#![cfg(test)]

use crate::{contract_a, ContractB, ContractBClient};
use soroban_sdk::{Env, Error};

#[test]
fn test() {
    let env = Env::default();

    // Register contract A using the imported WASM.
    let contract_a_id = env.register(contract_a::WASM, ());

    // Register contract B defined in this crate.
    let contract_b_id = env.register(ContractB, ());

    // Create a client for calling contract B.
    let client = ContractBClient::new(&env, &contract_b_id);

    // Invoke contract B via its client. Contract B will invoke contract A.
    let sum = client.add_with(&contract_a_id, &5, &7);
    assert_eq!(sum, 12);
}

#[test]
fn test_overflow() {
    let env = Env::default();

    let contract_a_id = env.register(contract_a::WASM, ());
    let contract_b_id = env.register(ContractB, ());

    let client = ContractBClient::new(&env, &contract_b_id);
    let res = client.try_add_with(&contract_a_id, &u32::MAX, &1);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            contract_a::Error::Overflow as u32
        )))
    );

    // Also verify calling contract A directly via its imported client
    let client_a = contract_a::Client::new(&env, &contract_a_id);
    let res_a = client_a.try_add(&u32::MAX, &1);
    assert_eq!(res_a, Err(Ok(contract_a::Error::Overflow)));
}
