#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, value: Symbol) {
        log!(&env, "Hello {}", value);
    }
}

mod test;
