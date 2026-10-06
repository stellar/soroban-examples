#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

/// A simple greeting contract that returns a hello message.
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    /// Returns a greeting for the given name.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment.
    /// * `to` - The name to greet.
    ///
    /// # Returns
    ///
    /// A vector containing two strings: `"Hello"` and the provided name.
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
