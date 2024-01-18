//! This is the 'timelock' example modified slightly to demonstrate
//! Soroban's fuzzing capabilities.
//!
//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]
pub struct ClaimableBalanceContract;

// The 'timelock' part: check that provided timestamp is before/after
// the current ledger timestamp.
fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
        TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
    }
}

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        // Perhaps this check should be enabled...
        /*if amount == 0 {
            panic!("deposit amount must not be zero");
        }*/

        if claimants.is_empty() {
            panic!("need more than 0 claimants");
        }
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }
        // Make sure `from` address authorized the deposit call with all the
        // arguments.
        from.require_auth();

        // Transfer token from `from` to this contract address.
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
        // Store all the necessary info to allow one of the claimants to claim it.
        env.storage().persistent().set(
            &DataKey::Balance,
            &ClaimableBalance {
                token,
                amount,
                time_bound,
                claimants,
            },
        );
        // Mark contract as initialized to prevent double-usage.
        // Note, that this is just one way to approach initialization - it may
        // be viable to allow one contract to manage several claimable balances.
        env.storage().persistent().set(&DataKey::Init, &());
    }

    pub fn claim(env: Env, claimant: Address, amount: i128) {
        // Make sure claimant has authorized this call, which ensures their
        // identity.
        claimant.require_auth();

        let mut claimable_balance: ClaimableBalance =
            env.storage().persistent().get(&DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant) {
            panic!("claimant is not allowed to claim this balance");
        }

        if amount > claimable_balance.amount {
            panic!("claimed amount greater than balance");
        }

        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &amount,
        );

        let new_balance = claimable_balance.amount - amount;

        if new_balance > 0 {
            // Store the new balance.
            claimable_balance.amount = new_balance;
            env.storage()
                .persistent()
                .set(&DataKey::Balance, &claimable_balance);
        } else {
            // Remove the balance entry to prevent any further claims.
            env.storage().persistent().remove(&DataKey::Balance);
        }
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().persistent().has(&DataKey::Init)
}

mod proptest;
