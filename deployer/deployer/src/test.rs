#![cfg(test)]
extern crate alloc;
extern crate std;

use crate::{Deployer, DeployerClient};
use alloc::vec;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    xdr::{self, ContractIdPreimage, ContractIdPreimageFromAddress, CreateContractArgs, Uint256},
    Address, BytesN, Env, IntoVal, Val, Vec,
};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(
        file =
            "../contract/target/wasm32-unknown-unknown/release/soroban_deployer_test_contract.wasm"
    );
}

#[test]
fn test_deploy_from_contract() {
    let env = Env::default();
    let deployer_client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(contract::WASM);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn = symbol_short!("init");
    let init_fn_args: Vec<Val> = (5u32,).into_val(&env);
    env.mock_all_auths();
    let (contract_id, init_result) = deployer_client.deploy(
        &deployer_client.address,
        &wasm_hash,
        &salt,
        &init_fn,
        &init_fn_args,
    );

    assert!(init_result.is_void());
    // No authorizations needed - the contract acts as a factory.
    assert_eq!(env.auths(), vec![]);

    // Invoke contract to check that it is initialized.
    let client = contract::Client::new(&env, &contract_id);
    let sum = client.value();
    assert_eq!(sum, 5);
}

#[test]
fn test_deploy_from_address() {
    let env = Env::default();
    let deployer_client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(contract::WASM);

    // Define a deployer address that needs to authorize the deployment.
    let deployer = Address::generate(&env);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn = symbol_short!("init");
    let init_fn_args: Vec<Val> = (5u32,).into_val(&env);
    env.mock_all_auths();
    let (contract_id, init_result) =
        deployer_client.deploy(&deployer, &wasm_hash, &salt, &init_fn, &init_fn_args);

    assert!(init_result.is_void());

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            deployer_client.address,
            symbol_short!("deploy"),
            (
                deployer.clone(),
                wasm_hash.clone(),
                salt,
                init_fn,
                init_fn_args,
            )
                .into_val(&env),
        )),
        // From `deploy` function the 'create contract' host function has to be
        // authorized.
        sub_invocations: vec![AuthorizedInvocation {
            function: AuthorizedFunction::CreateContractHostFn(CreateContractArgs {
                contract_id_preimage: ContractIdPreimage::Address(ContractIdPreimageFromAddress {
                    address: deployer.clone().try_into().unwrap(),
                    salt: Uint256([0; 32]),
                }),
                executable: xdr::ContractExecutable::Wasm(xdr::Hash(wasm_hash.into_val(&env))),
            }),
            sub_invocations: vec![],
        }],
    };
    assert_eq!(env.auths(), vec![(deployer, expected_auth)]);

    // Invoke contract to check that it is initialized.
    let client = contract::Client::new(&env, &contract_id);
    let sum = client.value();
    assert_eq!(sum, 5);
}
