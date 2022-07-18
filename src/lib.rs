#![no_std]

#[cfg(feature = "external")]
extern crate std;

pub mod external;
mod test;
mod token_contract;

use crate::token_contract::create_contract;
use stellar_contract_sdk::{
    contractimpl, BigInt, Binary, Env, EnvVal, IntoEnvVal, RawVal, Symbol, Vec,
};
use stellar_token_contract::public_types::{Authorization, Identifier, KeyedAuthorization, U256};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    TokenA = 0,
    TokenB = 1,
    TokenShare = 2,
    TotalShares = 3,
    ReserveA = 4,
    ReserveB = 5,
}

impl IntoEnvVal<Env, RawVal> for DataKey {
    fn into_env_val(self, env: &Env) -> EnvVal {
        (self as u32).into_env_val(env)
    }
}

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract().into())
}

fn get_token_a(e: &Env) -> Binary {
    e.get_contract_data(DataKey::TokenA)
}

fn get_token_b(e: &Env) -> Binary {
    e.get_contract_data(DataKey::TokenB)
}

fn get_token_share(e: &Env) -> Binary {
    e.get_contract_data(DataKey::TokenShare)
}

fn get_total_shares(e: &Env) -> BigInt {
    e.get_contract_data(DataKey::TotalShares)
}

fn get_reserve_a(e: &Env) -> BigInt {
    e.get_contract_data(DataKey::ReserveA)
}

fn get_reserve_b(e: &Env) -> BigInt {
    e.get_contract_data(DataKey::ReserveB)
}

fn get_balance(e: &Env, contract_id: Binary) -> BigInt {
    let mut args: Vec<EnvVal> = Vec::new(&e);
    args.push(get_contract_id(e).into_env_val(&e));
    e.call(contract_id, Symbol::from_str("balance"), args)
}

fn get_balance_a(e: &Env) -> BigInt {
    get_balance(&e, get_token_a(&e))
}

fn get_balance_b(e: &Env) -> BigInt {
    get_balance(&e, get_token_b(&e))
}

fn get_balance_shares(e: &Env) -> BigInt {
    get_balance(&e, get_token_share(&e))
}

fn put_token_a(e: &Env, contract_id: U256) {
    e.put_contract_data(DataKey::TokenA, contract_id);
}

fn put_token_b(e: &Env, contract_id: U256) {
    e.put_contract_data(DataKey::TokenB, contract_id);
}

fn put_token_share(e: &Env, contract_id: U256) {
    e.put_contract_data(DataKey::TokenShare, contract_id);
}

fn put_total_shares(e: &Env, amount: BigInt) {
    e.put_contract_data(DataKey::TotalShares, amount)
}

fn put_reserve_a(e: &Env, amount: BigInt) {
    e.put_contract_data(DataKey::ReserveA, amount)
}

fn put_reserve_b(e: &Env, amount: BigInt) {
    e.put_contract_data(DataKey::ReserveB, amount)
}

fn burn_shares(e: &Env, amount: BigInt) {
    let total = get_total_shares(e);

    let mut args = Vec::new(e);
    args.push(Authorization::Contract.into_env_val(e));
    args.push(get_contract_id(e).into_env_val(e));
    args.push(amount.clone().into_env_val(e));

    let share_contract_id = get_token_share(e);
    e.call::<()>(share_contract_id, Symbol::from_str("burn"), args);
    put_total_shares(e, total - amount);
}

fn mint_shares(e: &Env, to: Identifier, amount: BigInt) {
    let total = get_total_shares(e);

    let mut args = Vec::new(e);
    args.push(Authorization::Contract.into_env_val(e));
    args.push(to.into_env_val(e));
    args.push(amount.clone().into_env_val(e));

    let share_contract_id = get_token_share(e);
    e.call::<()>(share_contract_id, Symbol::from_str("mint"), args);
    put_total_shares(e, total + amount);
}

fn transfer(e: &Env, contract_id: Binary, to: Identifier, amount: BigInt) {
    let mut args = Vec::new(e);
    args.push(KeyedAuthorization::Contract.into_env_val(e));
    args.push(to.into_env_val(e));
    args.push(amount.into_env_val(e));
    e.call::<()>(contract_id, Symbol::from_str("xfer"), args);
}

fn transfer_a(e: &Env, to: Identifier, amount: BigInt) {
    transfer(&e, get_token_a(&e), to, amount);
}

fn transfer_b(e: &Env, to: Identifier, amount: BigInt) {
    transfer(&e, get_token_b(&e), to, amount);
}

pub trait LiquidityPoolTrait {
    fn initialize(e: Env, token_a: U256, token_b: U256);

    fn share_id(e: Env) -> Binary;

    fn deposit(e: Env, to: Identifier);

    fn swap(e: Env, to: Identifier, out_a: BigInt, out_b: BigInt);

    fn withdraw(e: Env, to: Identifier);
}

struct LiquidityPool;

#[contractimpl("export")]
impl LiquidityPoolTrait for LiquidityPool {
    fn initialize(e: Env, token_a: U256, token_b: U256) {
        if token_a > token_b {
            panic!();
        }

        let share_contract_id = create_contract(&e, &token_a, &token_b);
        let mut args = Vec::new(&e);
        args.push(get_contract_id(&e).into_env_val(&e));
        e.call::<()>(
            share_contract_id.clone().into(),
            Symbol::from_str("initialize"),
            args,
        );

        put_token_a(&e, token_a);
        put_token_b(&e, token_b);
        put_token_share(&e, share_contract_id.try_into().unwrap());
        put_total_shares(&e, BigInt::from_u32(&e, 0));
        put_reserve_a(&e, BigInt::from_u32(&e, 0));
        put_reserve_b(&e, BigInt::from_u32(&e, 0));
    }

    fn share_id(e: Env) -> Binary {
        get_token_share(&e)
    }

    fn deposit(e: Env, to: Identifier) {
        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));
        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));
        let total_shares = get_total_shares(&e);

        let zero = BigInt::from_u32(&e, 0);
        let new_total_shares = if reserve_a > zero.clone() && reserve_b > zero {
            let shares_a = (balance_a.clone() * total_shares.clone()) / reserve_a;
            let shares_b = (balance_b.clone() * total_shares.clone()) / reserve_b;
            shares_a.min(shares_b)
        } else {
            (balance_a.clone() * balance_b.clone()).sqrt()
        };

        mint_shares(&e, to, new_total_shares - total_shares);
        put_reserve_a(&e, balance_a);
        put_reserve_b(&e, balance_b);
    }

    fn swap(e: Env, to: Identifier, out_a: BigInt, out_b: BigInt) {
        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));
        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));

        // residue_numerator and residue_denominator are the amount that the invariant considers after
        // deducting the fee, scaled up by 1000 to avoid fractions
        let residue_numerator = BigInt::from_u32(&e, 997);
        let residue_denominator = BigInt::from_u32(&e, 1000);
        let zero = BigInt::from_u32(&e, 0);

        let new_invariant_factor = |balance: BigInt, reserve: BigInt, out: BigInt| {
            let delta = balance - reserve.clone() - out;
            let adj_delta = if delta > zero {
                residue_numerator.clone() * delta
            } else {
                residue_denominator.clone() * delta
            };
            residue_denominator.clone() * reserve + adj_delta
        };
        let new_inv_a = new_invariant_factor(balance_a.clone(), reserve_a.clone(), out_a.clone());
        let new_inv_b = new_invariant_factor(balance_b.clone(), reserve_b.clone(), out_b.clone());
        let old_inv_a = residue_denominator.clone() * reserve_a.clone();
        let old_inv_b = residue_denominator.clone() * reserve_b.clone();
        if new_inv_a * new_inv_b < old_inv_a * old_inv_b {
            panic!();
        }

        transfer_a(&e, to.clone(), out_a.clone());
        transfer_b(&e, to, out_b.clone());
        put_reserve_a(&e, balance_a - out_a);
        put_reserve_b(&e, balance_b - out_b);
    }

    fn withdraw(e: Env, to: Identifier) {
        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));
        let balance_shares = get_balance_shares(&e);
        let total_shares = get_total_shares(&e);

        let out_a = (balance_a.clone() * balance_shares.clone()) / total_shares.clone();
        let out_b = (balance_b.clone() * balance_shares.clone()) / total_shares.clone();

        burn_shares(&e, balance_shares);
        transfer_a(&e, to.clone(), out_a.clone());
        transfer_b(&e, to, out_b.clone());
        put_reserve_a(&e, balance_a - out_a);
        put_reserve_b(&e, balance_b - out_b);
    }
}
