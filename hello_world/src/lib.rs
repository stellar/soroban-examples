#![no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec};

pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), to]
    }
}

mod test;
