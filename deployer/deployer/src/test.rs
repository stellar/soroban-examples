#![cfg(test)]

use crate::{Deployer, DeployerClient};
use soroban_sdk::{Bytes, Env, IntoVal, Symbol};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/soroban_deployer_test_contract.wasm"
    );
}

#[test]
fn test() {
    let env = Env::default();
    let client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    // Install the WASM code to be deployed from the deployer contract.
    let wasm_hash = env.install_contract_wasm(contract::WASM);

    // Deploy contract using deployer, and include an init function to call.
    let salt = Bytes::from_array(&env, &[0; 32]);
    let init_fn = Symbol::short("init");
    let init_fn_args = (5u32,).into_val(&env);
    let (contract_id, init_result) = client.deploy(&salt, &wasm_hash, &init_fn, &init_fn_args);
    assert!(init_result.is_void());

    // Invoke contract to check that it is initialized.
    let client = contract::Client::new(&env, &contract_id);
    let sum = client.value();
    assert_eq!(sum, 5);
}
