#![no_std]

use soroban_sdk::{contractimpl, BytesN, Env};

mod contract_a {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/soroban_cross_contract_a_contract.wasm"
    );
}

pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract_id: BytesN<32>, x: u32, y: u32) -> u32 {
        let client = contract_a::Client::new(&env, &contract_id);
        client.add(&x, &y)
    }
}

mod test;
