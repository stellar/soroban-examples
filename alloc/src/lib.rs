#![no_std]
use soroban_sdk::{contractimpl, Env};

extern crate alloc;

pub struct AllocContract;

#[contractimpl]
impl AllocContract {
    /// Allocates a temporary vector holding values (0..count), then computes and returns their sum.
    pub fn sum(_env: Env, count: u32) -> u32 {
        let mut v1 = alloc::vec![];
        (0..count).for_each(|i| v1.push(i));

        let mut sum = 0;
        for i in v1 {
            sum += i;
        }

        sum
    }
}

mod test;
