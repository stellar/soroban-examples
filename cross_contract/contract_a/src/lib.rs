#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Overflow = 1,
}

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn add(x: u32, y: u32) -> Result<u32, Error> {
        x.checked_add(y).ok_or(Error::Overflow)
    }
}
