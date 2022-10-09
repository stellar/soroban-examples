#![no_std]

use soroban_auth::{
    verify, {Identifier, Signature},
};
use soroban_sdk::{contracterror, contractimpl, contracttype, panic_error, symbol, BigInt, Env};

#[contracttype]
pub enum DataKey {
    Counter(Identifier),
    Nonce(Identifier),
}

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IncorrectNonceForInvoker = 1,
    IncorrectNonce = 2,
}

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments a counter for the invoker, and returns the value.
    pub fn increment(env: Env, sig: Signature, nonce: BigInt) -> u32 {
        // Verify that the signature signs and authorizes this invocation.
        let id = sig.identifier(&env);
        verify(&env, &sig, symbol!("increment"), (&id, &nonce));

        // Verify that the nonce has not been consumed to prevent replay of the
        // same presigned invocation more than once.
        verify_and_consume_nonce(&env, &sig, &nonce);

        // Construct a key for the data being stored. Use an enum to set the
        // contract up well for adding other types of data to be stored.
        let key = DataKey::Counter(id);

        // Get the current count for the invoker.
        let mut count: u32 = env
            .data()
            .get(&key)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.

        // Increment the count.
        count += 1;

        // Save the count.
        env.data().set(&key, count);

        // Return the count to the caller.
        count
    }

    pub fn nonce(env: Env, id: Identifier) -> BigInt {
        get_nonce(&env, &id)
    }
}

fn verify_and_consume_nonce(env: &Env, auth: &Signature, nonce: &BigInt) {
    match auth {
        Signature::Invoker => {
            if BigInt::zero(env) != nonce {
                panic_error!(env, Error::IncorrectNonceForInvoker);
            }
        }
        Signature::Ed25519(_) | Signature::Account(_) => {
            let id = auth.identifier(env);
            if nonce != &get_nonce(env, &id) {
                panic_error!(env, Error::IncorrectNonce);
            }
            set_nonce(env, &id, nonce + 1);
        }
    }
}

fn get_nonce(env: &Env, id: &Identifier) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    env.data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(env)))
        .unwrap()
}

fn set_nonce(env: &Env, id: &Identifier, nonce: BigInt) {
    let key = DataKey::Nonce(id.clone());
    env.data().set(key, nonce);
}

mod test;
