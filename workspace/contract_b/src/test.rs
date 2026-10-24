#![cfg(test)]

use soroban_sdk::{Env, Error};

use crate::{ContractB, ContractBClient};

use soroban_workspace_contract_a::ContractA;
use soroban_workspace_contract_a_interface::{ContractAClient, Error as ContractAError};

#[test]
fn test() {
    let env = Env::default();

    // Register contract A using the native contract imported.
    let contract_a_id = env.register(ContractA, ());

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

    let contract_a_id = env.register(ContractA, ());
    let contract_b_id = env.register(ContractB, ());

    let client = ContractBClient::new(&env, &contract_b_id);
    let res = client.try_add_with(&contract_a_id, &u32::MAX, &1);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractAError::Overflow as u32
        )))
    );

    // Also verify calling contract A directly via ContractAClient
    let client_a = ContractAClient::new(&env, &contract_a_id);
    let res_a = client_a.try_add(&u32::MAX, &1);
    assert_eq!(res_a, Err(Ok(ContractAError::Overflow)));
}
