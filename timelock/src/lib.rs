//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.
#![no_std]

use soroban_sdk::{contractimpl, contracttype, BytesN, Env, Vec};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

use token::{Identifier, Signature};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
}

#[derive(Clone)]
#[contracttype]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: BytesN<32>,
    pub amount: i128,
    pub claimants: Vec<Identifier>,
    pub time_bound: TimeBound,
}

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

// Contract usage pattern (pseudocode):
// 1. Depositor calls `token.approve(depositor_auth, claimable_balance_contract, 100)`
//    to allow contract to withdraw the needed amount of token.
// 2. Depositor calls `deposit(token_id, 100, claimants, time bound)`. Contract
//    withdraws the provided token amount and stores it until one of the claimants
//    claims it.
// 3. Claimant calls `claim()` and if time/auth conditons are passed
//    receives the balance.
#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        token: BytesN<32>,
        amount: i128,
        claimants: Vec<Identifier>,
        time_bound: TimeBound,
    ) {
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }
        if amount < 0 {
            panic!("negative amount is not allowed")
        }

        // Transfer token to this contract address.
        transfer_from_account_to_contract(&env, &token, &env.invoker().into(), &amount);
        // Store all the necessary info to allow one of the claimants to claim it.
        env.data().set(
            DataKey::Balance,
            ClaimableBalance {
                token,
                amount,
                time_bound,
                claimants,
            },
        );
        // Mark contract as initialized to prevent double-usage.
        // Note, that this is just one way to approach initialization - it may
        // be viable to allow one contract to manage several claimable balances.
        env.data().set(DataKey::Init, ());
    }

    pub fn claim(env: Env) {
        let claimable_balance: ClaimableBalance =
            env.data().get_unchecked(DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimant_id = env.invoker().into();
        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant_id) {
            panic!("claimant is not allowed to claim this balance");
        }

        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        transfer_from_contract_to_account(
            &env,
            &claimable_balance.token,
            &claimant_id,
            &claimable_balance.amount,
        );
        // Remove the balance entry to prevent any further claims.
        env.data().remove(DataKey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.data().has(DataKey::Init)
}

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract())
}

fn transfer_from_account_to_contract(
    e: &Env,
    token_id: &BytesN<32>,
    from: &Identifier,
    amount: &i128,
) {
    let client = token::Client::new(e, token_id);
    client.xfer_from(&Signature::Invoker, &0, from, &get_contract_id(e), amount);
}

fn transfer_from_contract_to_account(
    e: &Env,
    token_id: &BytesN<32>,
    to: &Identifier,
    amount: &i128,
) {
    let client = token::Client::new(e, token_id);
    client.xfer(&Signature::Invoker, &0, to, amount);
}

mod test;
