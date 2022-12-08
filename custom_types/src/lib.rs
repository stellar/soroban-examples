#![no_std]
use soroban_sdk::{contractimpl, contracttype, symbol, Env, Symbol};

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

const STATE: Symbol = symbol!("STATE");

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env, incr: u32) -> u32 {
        // Get the current count.
        let mut state = Self::get_state(env.clone());

        // Increment the count.
        state.count += incr;
        state.last_incr = incr;

        // Save the count.
        env.storage().set(STATE, &state);

        // Return the count to the caller.
        state.count
    }
    /// Return the current state.
    pub fn get_state(env: Env) -> State {
        env.storage()
            .get(STATE)
            .unwrap_or_else(|| Ok(State::default())) // If no value set, assume 0.
            .unwrap() // Panic if the value of COUNTER is not a State.
    }
}

mod test;
