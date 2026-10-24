#![no_std]

mod admin;
mod allowance;
mod balance;
mod contract;
pub mod error;
mod metadata;
mod storage_types;
mod test;

pub use crate::contract::TokenClient;
pub use crate::error::{Error, TokenError};
