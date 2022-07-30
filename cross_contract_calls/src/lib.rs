#![no_std]
use soroban_sdk::{contractimpl, vec, Env, FixedBinary, IntoVal, Symbol};

pub struct ContractA;

#[contractimpl(export_if = "export")]
impl ContractA {
    pub fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }
}

pub struct ContractB;

#[contractimpl(export_if = "export")]
impl ContractB {
    pub fn add_with(env: Env, x: u32, y: u32, contract_id: FixedBinary<32>) -> u32 {
        env.invoke_contract(
            &contract_id,
            &Symbol::from_str("add"),
            vec![&env, x.into_env_val(&env), y.into_env_val(&env)],
        )
    }
}

mod test;
