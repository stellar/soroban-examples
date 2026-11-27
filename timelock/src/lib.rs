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
    /// Marks whether the contract has been initialized.
    Init,
    /// Stores the claimable balance details.
    Balance,
}

/// Defines whether the timelock expires before or after a given timestamp.
#[derive(Clone)]
#[contracttype]
pub enum TimeBoundKind {
    /// The claim is only valid before the specified timestamp.
    Before,
    /// The claim is only valid after the specified timestamp.
    After,
}

/// A time constraint that restricts when a claimable balance can be claimed.
#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    /// Specifies whether the claim must occur before or after the timestamp.
    pub kind: TimeBoundKind,
    /// The Unix timestamp (in seconds) used for the time constraint.
    pub timestamp: u64,
}

/// Holds the state of a deposited claimable balance.
#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    /// The address of the token contract.
    pub token: Address,
    /// The token amount that has been deposited and can be claimed.
    pub amount: i128,
    /// The list of addresses that are allowed to claim the balance.
    pub claimants: Vec<Address>,
    /// The time constraint that determines when the balance can be claimed.
    pub time_bound: TimeBound,
}

/// A contract that implements a claimable balance with a time lock.
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
    /// Deposits tokens into the contract to create a claimable balance.
    ///
    /// The balance can later be claimed by one of the specified claimants,
    /// provided the time constraint is satisfied.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment.
    /// * `from` - The address of the depositor. Must authorize this call.
    /// * `token` - The address of the token contract.
    /// * `amount` - The number of tokens to deposit.
    /// * `claimants` - A list of addresses eligible to claim the balance (max 10).
    /// * `time_bound` - The time constraint that must be satisfied before the
    ///   balance can be claimed.
    ///
    /// # Panics
    ///
    /// Panics if more than 10 claimants are provided, or if the contract has
    /// already been initialized.
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
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
        env.storage().instance().set(
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
        env.storage().instance().set(&DataKey::Init, &());
    }

    /// Claims the deposited balance on behalf of an authorized claimant.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment.
    /// * `claimant` - The address attempting to claim. Must be in the claimants
    ///   list and must authorize this call.
    ///
    /// # Panics
    ///
    /// Panics if the time constraint is not satisfied, if the claimant is not
    /// in the allowed list, or if there is no balance to claim.
    pub fn claim(env: Env, claimant: Address) {
        // Make sure claimant has authorized this call, which ensures their
        // identity.
        claimant.require_auth();
        // Just get the balance - if it's been claimed, this will simply panic
        // and terminate the contract execution.
        let claimable_balance: ClaimableBalance =
            env.storage().instance().get(&DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant) {
            panic!("claimant is not allowed to claim this balance");
        }

        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );
        // Remove the balance entry to prevent any further claims.
        env.storage().instance().remove(&DataKey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Init)
}

mod test;
