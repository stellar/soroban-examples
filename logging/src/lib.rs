#![no_std]
use soroban_sdk::{contractimpl, log, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, value: Symbol) {
        log!(&env, "Hello {}", value);
    }
}

mod test;
