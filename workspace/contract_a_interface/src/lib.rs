#![no_std]

use soroban_sdk::contractclient;

#[contractclient(name = "ContractAClient")]
pub trait ContractAInterface {
    fn add(x: u32, y: u32) -> u32;
}
