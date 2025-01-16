#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const PAUSED: Symbol = symbol_short!("PAUSED");

#[contract]
pub struct Pause;

#[contractimpl]
impl Pause {
    pub fn paused(env: Env) -> bool {
        env.storage().instance().get(&PAUSED).unwrap_or_default()
    }

    pub fn set(env: Env, paused: bool) {
        env.storage().instance().set(&PAUSED, &paused);
    }
}

mod test;
