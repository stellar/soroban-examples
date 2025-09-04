#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

// Define two static topics for the event: "COUNTER" and "increment".
// Also set the data format to "single-value", which means that the event data
// payload will contain a single value not nested into any data structure.
#[contractevent(topics = ["COUNTER", "increment"], data_format = "single-value")]
struct IncrementEvent {
    count: u32,
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // Publish an event about the increment occuring.
        // The event has two static topics ("COUNTER", "increment") and actual
        // count as the data payload.
        IncrementEvent { count }.publish(&env);

        // Return the count to the caller.
        count
    }
}

mod test;
