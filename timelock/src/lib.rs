//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.
#![no_std]

use soroban_sdk::{contractimpl, contracttype, vec, Account, Address, BytesN, Env, IntoVal, Vec};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

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
    pub claimants: Vec<Address>,
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

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Account,
        token: BytesN<32>,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        from.authorize((&token, &amount, &claimants, &time_bound).into_val(&env));
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }

        // Transfer token to this contract address.
        transfer_from_account_to_contract(&env, &token, &from, &amount);
        // Store all the necessary info to allow one of the claimants to claim it.
        env.storage().set(
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
        env.storage().set(DataKey::Init, ());
    }

    pub fn claim(env: Env, claimant: Account) {
        claimant.authorize(vec![&env]);

        let claimable_balance: ClaimableBalance =
            env.storage().get_unchecked(DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant.address()) {
            panic!("claimant is not allowed to claim this balance");
        }

        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        transfer_from_contract_to_account(
            &env,
            &claimable_balance.token,
            &claimant.address(),
            &claimable_balance.amount,
        );
        // Remove the balance entry to prevent any further claims.
        env.storage().remove(DataKey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().has(DataKey::Init)
}

fn transfer_from_account_to_contract(
    e: &Env,
    token_id: &BytesN<32>,
    from: &Account,
    amount: &i128,
) {
    let client = token::Client::new(&e, token_id);
    client.xfer(from, &e.current_contract_account().address(), amount);
}

fn transfer_from_contract_to_account(e: &Env, token_id: &BytesN<32>, to: &Address, amount: &i128) {
    let client = token::Client::new(&e, token_id);
    client.xfer(&e.current_contract_account(), to, amount);
}

mod test;
