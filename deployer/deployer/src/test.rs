#![cfg(test)]

use crate::{Deployer, DeployerClient};
use soroban_sdk::{bytesn, symbol, Bytes, BytesN, Env, IntoVal};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/soroban_deployer_test_contract.wasm"
    );
}

#[test]
fn test() {
    let env = Env::default();
    let deployer_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&deployer_id, Deployer);
    let client = DeployerClient::new(&env, &deployer_id);

    // Deploy contract using deployer, and include an init function to call.
    let salt = Bytes::from_array(&env, &[0; 32]);
    let wasm: Bytes = contract::WASM.into_val(&env);
    let init_fn = symbol!("init");
    let init_fn_args = (5u32,).into_val(&env);
    let (contract_id, init_result) = client.deploy(&salt, &wasm, &init_fn, &init_fn_args);
    assert_eq!(
        contract_id,
        bytesn!(&env, 0xead19f55aec09bfcb555e09f230149ba7f72744a5fd639804ce1e934e8fe9c5d)
    );
    assert!(init_result.is_void());

    // Invoke contract to check that it is initialized.
    let client = contract::Client::new(&env, &contract_id);
    let sum = client.value();
    assert_eq!(sum, 5);
}
