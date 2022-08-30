#![no_std]
use soroban_sdk::{contractimpl, map, Env, Symbol};

pub struct EventContract;

#[contractimpl]
impl EventContract {
    pub fn hello(env: Env, to: Symbol) -> () {
        const GREETING: Symbol = Symbol::from_str("Hello");
        let event = env.events();
        let topics = (GREETING, to);
        let data = map![&env, (1u32, 2u32)];
        event.publish(topics, data);
    }
}

mod test;
