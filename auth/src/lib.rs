#![no_std]
use soroban_sdk::{contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Counter(Address),
}

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments a counter for the invoker, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Construct a key for the data being stored. Use an enum to set the
        // contract up well for adding other types of data to be stored.
        let invoker = env.invoker();
        let key = DataKey::Counter(invoker);

        // Get the current count for the invoker.
        let mut count: u32 = env
            .storage()
            .get(&key)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().set(&key, count);

        // Return the count to the caller.
        count
    }
}

mod test;
