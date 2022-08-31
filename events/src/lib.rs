#![no_std]
use soroban_sdk::{contractimpl, map, symbol, Env, Symbol};

pub struct EventsContract;

#[contractimpl]
impl EventsContract {
    pub fn hello(env: Env, to: Symbol) -> () {
        let events = env.events();
        let topics = (symbol!("Hello"), to);
        let data = map![&env, (1u32, 2u32)];
        events.publish(topics, data);
    }
}

mod test;
