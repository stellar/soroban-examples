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

/// A counter contract that emits a Soroban event each time the counter is incremented.
#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increments an internal counter and emits a `COUNTER/increment` event.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment.
    ///
    /// # Returns
    ///
    /// The new counter value after incrementing.
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
