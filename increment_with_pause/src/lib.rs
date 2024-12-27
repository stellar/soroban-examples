#![no_std]
use soroban_sdk::{
    contract, contractclient, contracterror, contractimpl, symbol_short, Address, Env, Symbol,
};

const COUNTER: Symbol = symbol_short!("COUNTER");
const PAUSE: Symbol = symbol_short!("PAUSE");

#[contract]
pub struct IncrementContract;

#[contracterror]
#[derive(PartialEq, Debug)]
pub enum Error {
    Paused = 1,
}

#[contractclient(name = "PauseClient")]
pub trait Pause {
    fn paused(env: Env) -> bool;
}

#[contractimpl]
impl IncrementContract {
    pub fn __constructor(env: Env, pause: Address) {
        env.storage().instance().set(&PAUSE, &pause);
    }

    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> Result<u32, Error> {
        let pause_address: Address = env.storage().instance().get(&PAUSE).unwrap();
        let pause = PauseClient::new(&env, &pause_address);

        if pause.paused() {
            return Err(Error::Paused);
        }

        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // The contract instance will be bumped to have a lifetime of at least 100 ledgers if the current expiration lifetime at most 50.
        // If the lifetime is already more than 100 ledgers, this is a no-op. Otherwise,
        // the lifetime is extended to 100 ledgers. This lifetime bump includes the contract
        // instance itself and all entries in storage().instance(), i.e, COUNTER.
        env.storage().instance().extend_ttl(50, 100);

        // Return the count to the caller.
        Ok(count)
    }
}

mod test_mock;
mod test_real;
