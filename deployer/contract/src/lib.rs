#![no_std]

use soroban_sdk::{contractimpl, Env, Symbol};

pub struct Contract;

const KEY: Symbol = Symbol::short("value");

#[contractimpl]
impl Contract {
    pub fn init(env: Env, value: u32) {
        env.storage().set(&KEY, &value);
    }
    pub fn value(env: Env) -> u32 {
        env.storage().get_unchecked(&KEY).unwrap()
    }
}
