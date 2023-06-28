#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

mod contract_a {
    soroban_sdk::contractimport!(
        file = "../contract_a/target/wasm32-unknown-unknown/release/soroban_cross_contract_a_contract.wasm"
    );
}

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract: Address, x: u32, y: u32) -> u32 {
        let client = contract_a::Client::new(&env, &contract);
        client.add(&x, &y)
    }
}

mod test;
