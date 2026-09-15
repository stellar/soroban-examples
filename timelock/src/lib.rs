//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

/// Storage keys for the claimable balance contract.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Tracks contract initialization status.
    Init,
    /// Stores the active claimable balance information.
    Balance,
}

/// Specifies the condition under which the balance may be claimed relative to a timestamp.
#[derive(Clone)]
#[contracttype]
pub enum TimeBoundKind {
    /// Balance can be claimed at or before the specified timestamp (inclusive: ledger_timestamp <= timestamp).
    Before,
    /// Balance can be claimed at or after the specified timestamp (inclusive: ledger_timestamp >= timestamp).
    After,
}

/// Defines a timestamp and whether claiming is permitted before or after that point.
#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    /// Condition type (Before or After).
    pub kind: TimeBoundKind,
    /// Target unix timestamp in seconds.
    pub timestamp: u64,
}

/// Holds information about an escrowed claimable balance.
#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    /// Address of the token asset being held.
    pub token: Address,
    /// Amount of tokens deposited in the claimable balance.
    pub amount: i128,
    /// List of addresses authorized to claim the deposit.
    pub claimants: Vec<Address>,
    /// Time condition required before a claim is allowed.
    pub time_bound: TimeBound,
}

/// Contract managing a single time-locked claimable balance.
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
    /// Deposits tokens into the contract and configures claimable balance rules.
    ///
    /// # Arguments
    /// * `from` - Depositor address providing the tokens (requires authorization).
    /// * `token` - Address of the token contract being deposited.
    /// * `amount` - Number of tokens to transfer and lock into the contract.
    /// * `claimants` - List of addresses eligible to claim the balance (max 10).
    /// * `time_bound` - Time condition specifying when the balance can be claimed.
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

    /// Claims the deposited tokens if the caller is an authorized claimant and the time bound is fulfilled.
    ///
    /// # Arguments
    /// * `claimant` - Authorized recipient claiming the tokens (requires authorization).
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
