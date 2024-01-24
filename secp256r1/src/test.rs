#![cfg(test)]

use super::*;
use soroban_sdk::{bytesn, Address, Env};

#[test]
fn test_native() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    test(&env, &contract_id);
}

#[test]
fn test_wasm() {
    const WASM: &[u8] =
        include_bytes!("../target/wasm32-unknown-unknown/release/soroban_secp256r1_contract.wasm");
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, WASM);
    test(&env, &contract_id);
}

fn test(env: &Env, contract_id: &Address) {
    let client = ContractClient::new(env, contract_id);

    let key = bytesn!(&env, 0x04f7281e35b4266fd87dfc808df32354d2dfd09a645c281a94ab70f3990cb09972ecdf72c1a8d92c24abe764ac895c8a7118cf580adf33ba7d7f44f2b4cf38bba7);
    let msg = bytesn!(
        &env,
        0x0000000000000000000000000000000000000000000000000000000000000000
    );
    let sig = bytesn!(&env, 0x723da9c4c292269fb8e87054acef4c6ca8d5ba31118b7e433ce78ee079ee80043b0ee5ca5963e542f53ff257d1bd7b9c886deaec798f01b73ae86c1763ef2c48);

    // Set an unlimited budget because it'll fail with the default budget.
    env.budget().reset_unlimited();

    let result = client.try_secp256r1_verify(&key, &msg, &sig);
    assert_eq!(result, Ok(Ok(())));

    env.budget().print();
}
