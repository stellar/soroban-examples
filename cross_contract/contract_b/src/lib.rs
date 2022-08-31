#![no_std]

use soroban_sdk::{contractimpl, vec, BytesN, Env, IntoVal, Symbol};

mod contract_a {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/soroban_cross_contract_a_contract.wasm"
    );
}

pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract_id: BytesN<32>, x: u32, y: u32) -> u32 {
        env.invoke_contract(
            &contract_id,
            &Symbol::from_str("add"),
            vec![&env, x.into_env_val(&env), y.into_env_val(&env)],
        )
    }
}

mod test;
