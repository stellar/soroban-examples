#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct Contract;

const KEY: Symbol = symbol_short!("value");

#[contractimpl]
impl Contract {
    pub fn init(env: Env, value: u32) {
        env.storage().instance().set(&KEY, &value);
    }
    pub fn value(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap()
    }
}
