//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
#![no_std]
#[cfg(feature = "testutils")]
extern crate std;

use soroban_auth::{
    verify, {Identifier, Signature},
};
use soroban_sdk::{contractimpl, contracttype, BigInt, BytesN, Env, Symbol, Vec};

mod token {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
    Nonce(Identifier),
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
    pub amount: BigInt,
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
// 2. Depositor calls `deposit(depositor_auth, token_id, 100, claimants, time bound)`. Contract
//    withdraws the provided token amount and stores it until one of the claimants
//    claims it.
// 3. Claimant calls `claim(claimant_auth)` and if time/auth conditons are passed
//    receives the balance.
#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Signature,
        token: BytesN<32>,
        amount: BigInt,
        claimants: Vec<Identifier>,
        time_bound: TimeBound,
    ) {
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }

        let from_id = from.identifier(&env);

        verify_and_consume_nonce(&env, &from_id, &BigInt::zero(&env));

        // Authenticate depositor with nonce of zero, so that this may
        // be successfully called just once.
        verify(
            &env,
            &from,
            Symbol::from_str("deposit"),
            (&from_id, &token, &amount, &claimants, &time_bound),
        );
        // Transfer token to this contract address.
        transfer_from(&env, &token, &from_id, &get_contract_id(&env), &amount);
        // Store all the necessary balance to allow one of the claimants to claim it.
        env.data().set(
            DataKey::Balance,
            ClaimableBalance {
                token,
                amount,
                time_bound,
                claimants,
            },
        );
        env.data().set(DataKey::Init, ());
    }

    pub fn claim(env: Env, claimant: Signature) {
        let claimable_balance: ClaimableBalance =
            env.data().get_unchecked(DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimant_id = claimant.identifier(&env);
        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant_id) {
            panic!("claimant is not allowed to claim this balance");
        }

        verify_and_consume_nonce(&env, &claimant_id, &BigInt::zero(&env));

        // Authenticate claimant with nonce of zero, so that this may be
        // successfully called just once.
        // For simplicity, depositor can't be the claimant.
        verify(&env, &claimant, Symbol::from_str("claim"), (&claimant_id,));
        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        transfer_to(
            &env,
            &claimable_balance.token,
            &claimant_id,
            &claimable_balance.amount,
        );
        // Cleanup unnecessary balance entry.
        env.data().remove(DataKey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.data().has(DataKey::Init)
}

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract().into())
}

fn transfer_from(
    e: &Env,
    token_id: &BytesN<32>,
    from: &Identifier,
    to: &Identifier,
    amount: &BigInt,
) {
    let client = token::Client::new(&e, token_id);
    client.xfer_from(&Signature::Invoker, &BigInt::zero(&e), &from, &to, &amount);
}

fn transfer_to(e: &Env, token_id: &BytesN<32>, to: &Identifier, amount: &BigInt) {
    let client = token::Client::new(&e, token_id);
    client.xfer(&Signature::Invoker, &BigInt::zero(&e), to, amount);
}

fn read_nonce(e: &Env, id: &Identifier) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    e.data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(e)))
        .unwrap()
}

fn verify_and_consume_nonce(e: &Env, id: &Identifier, expected_nonce: &BigInt) {
    match id {
        Identifier::Contract(_) => {
            if BigInt::zero(&e) != expected_nonce {
                panic!("nonce should be zero for Contract")
            }
            return;
        }
        _ => {}
    }

    let key = DataKey::Nonce(id.clone());
    let nonce = read_nonce(e, id);

    if nonce != expected_nonce {
        panic!("incorrect nonce")
    }
    e.data().set(key, &nonce + 1);
}

mod test;
