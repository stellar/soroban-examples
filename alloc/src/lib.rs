use soroban_sdk::{contractimpl, symbol, Env, Symbol, Vec};

extern crate alloc;

const VECTOR: Symbol = symbol!("VECTOR");

pub struct AllocContract;

#[contractimpl]
impl AllocContract {
    pub fn init(env: Env) {
        let v: Vec<u32> = Vec::new(&env);
        env.storage().set(VECTOR, v)
    }

    /// Allocates a temporary vector holding values (0..count), pushes it to the back of the
    /// internal vector, and returns its new length.
    pub fn grow(env: Env, count: u32) -> u32 {
        let mut v1 = alloc::vec![];
        (0..count).for_each(|i| v1.push(i));

        // Get the current vector and grow it.
        let mut vec: Vec<u32> = env.storage().get(VECTOR).unwrap().unwrap(); // Panic if the vector doesn't exist or has the wrong type
        for i in v1 {
            vec.push_back(i);
        }

        // Record the new length and save the vector.
        let len = vec.len();
        env.storage().set(VECTOR, vec);

        // Return the new length to the caller.
        len
    }
}

mod test;
