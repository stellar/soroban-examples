#![no_std]

use soroban_sdk::{contract, contractimpl};
use soroban_workspace_contract_a_interface::{ContractAInterface, Error};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractAInterface for ContractA {
    fn add(x: u32, y: u32) -> Result<u32, Error> {
        x.checked_add(y).ok_or(Error::Overflow)
    }
}
