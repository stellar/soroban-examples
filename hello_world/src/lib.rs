#![no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec};

pub struct HelloContract;

#[contractimpl(export_if = "export")]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        const GREETING: Symbol = Symbol::from_str("Hello");
        vec![&env, GREETING, to]
    }
}

mod test;
