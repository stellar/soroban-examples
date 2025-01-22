#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
}

#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimant: Address,
    pub time_bound: TimeBound,
}

#[contract]
pub struct TokenTimelockContract;

fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();
    ledger_timestamp >= time_bound.timestamp
}

#[contractimpl]
impl TokenTimelockContract {
    pub fn __constructor(env: Env, token: Address, amount: i128, claimant: Address, time_bound: TimeBound) {
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }

        token::Client::new(&env, &token).transfer(&env.invoker(), &env.current_contract_address(), &amount);

        env.storage().instance().set(
            &DataKey::Balance,
            &ClaimableBalance {
                token,
                amount,
                time_bound,
                claimant,
            },
        );

        env.storage().instance().set(&DataKey::Init, &());
    }

    pub fn claim(env: Env, claimant: Address) {
        claimant.require_auth();
        let claimable_balance: ClaimableBalance = env.storage().instance().get(&DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        if claimable_balance.claimant != claimant {
            panic!("claimant is not allowed to claim this balance");
        }

        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );

        env.storage().instance().remove(&DataKey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Init)
}

mod test;
