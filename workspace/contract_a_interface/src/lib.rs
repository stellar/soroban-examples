#![no_std]

use soroban_sdk::{contractclient, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Overflow = 1,
}

#[contractclient(name = "ContractAClient")]
pub trait ContractAInterface {
    fn add(x: u32, y: u32) -> Result<u32, Error>;
}
