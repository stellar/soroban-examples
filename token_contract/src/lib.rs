#![no_std]

#[cfg(any(test, feature = "testutils"))]
#[macro_use]
extern crate std;

mod admin;
mod allowance;
mod balance;
mod contract;
mod metadata;
mod storage_types;
mod test;
pub mod testutils;

pub use crate::contract::TokenClient;
