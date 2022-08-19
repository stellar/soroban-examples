#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod pool_contract;
mod test;
pub mod testutils;

use pool_contract::create_contract;
use soroban_liquidity_pool_contract as liquidity_pool;
use soroban_sdk::{contractimpl, contracttype, BigInt, Bytes, BytesN, Env};
use soroban_token_contract as token;
use token::public_types::{Identifier, KeyedAuthorization, U256};

pub use crate::get_pool::invoke as get_pool;
pub use crate::sf_deposit::invoke as sf_deposit;
pub use crate::sf_withdrw::invoke as sf_withdrw;
pub use crate::swap_out::invoke as swap_out;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Pool(U256),
}

fn get_pool_id(e: &Env, salt: &U256) -> U256 {
    e.contract_data()
        .get_unchecked(DataKey::Pool(salt.clone()))
        .unwrap()
}

fn put_pool(e: &Env, salt: &U256, pool: &U256) {
    e.contract_data()
        .set(DataKey::Pool(salt.clone()), pool.clone())
}

fn has_pool(e: &Env, salt: &U256) -> bool {
    e.contract_data().has(DataKey::Pool(salt.clone()))
}

pub trait LiquidityPoolRouterTrait {
    // TODO: Add auth for deposit, swap, and withdraw
    fn sf_deposit(
        e: Env,
        to: Identifier,
        token_a: U256,
        token_b: U256,
        desired_a: BigInt,
        min_a: BigInt,
        desired_b: BigInt,
        min_b: BigInt,
    );

    // swaps out an exact amount of "buy", in exchange for "sell" that this contract has an
    // allowance for from "to". "sell" amount swapped in must not be greater than "in_max"
    fn swap_out(e: Env, to: Identifier, sell: U256, buy: U256, out: BigInt, in_max: BigInt);

    fn sf_withdrw(
        e: Env,
        to: Identifier,
        token_a: U256,
        token_b: U256,
        share_amount: BigInt,
        min_a: BigInt,
        min_b: BigInt,
    );

    // returns the contract address for the specified token_a/token_b combo
    fn get_pool(e: Env, token_a: U256, token_b: U256) -> U256;
}

fn sort(a: &U256, b: &U256) -> (U256, U256) {
    if a < b {
        return (a.clone(), b.clone());
    } else if a > b {
        return (b.clone(), a.clone());
    }
    panic!("a and b can't be the same")
}

pub fn pool_salt(e: &Env, token_a: &U256, token_b: &U256) -> BytesN<32> {
    if token_a >= token_b {
        panic!("token_a must be less t&han token_b");
    }

    let mut salt_bin = Bytes::new(&e);
    salt_bin.append(&token_a.clone().into());
    salt_bin.append(&token_b.clone().into());
    e.compute_hash_sha256(salt_bin)
}

fn get_deposit_amounts(
    desired_a: BigInt,
    min_a: BigInt,
    desired_b: BigInt,
    min_b: BigInt,
    reserves: (BigInt, BigInt),
) -> (BigInt, BigInt) {
    if reserves.0 == 0 && reserves.1 == 0 {
        return (desired_a, desired_b);
    }

    let amount_b = desired_a.clone() * reserves.1.clone() / reserves.0.clone();
    if amount_b <= desired_b {
        if amount_b < min_b {
            panic!("amount_b less than min")
        }
        return (desired_a, amount_b);
    } else {
        let amount_a = desired_b.clone() * reserves.0 / reserves.1;
        if amount_a > desired_a || desired_a < min_a {
            panic!("amount_a invalid")
        }
        return (amount_a, desired_b);
    }
}

struct LiquidityPoolRouter;

#[contractimpl]
impl LiquidityPoolRouterTrait for LiquidityPoolRouter {
    fn sf_deposit(
        e: Env,
        to: Identifier,
        token_a: U256,
        token_b: U256,
        desired_a: BigInt,
        min_a: BigInt,
        desired_b: BigInt,
        min_b: BigInt,
    ) {
        let salt = pool_salt(&e, &token_a, &token_b);
        if !has_pool(&e, &salt) {
            let pool_contract_id = create_contract(&e, &salt);

            put_pool(&e, &salt, &pool_contract_id);

            liquidity_pool::initialize(&e, &pool_contract_id, &token_a, &token_b);
        }

        let pool_id = get_pool_id(&e, &salt);

        let reserves = liquidity_pool::get_rsrvs(&e, &pool_id);
        let amounts = get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserves);

        token::xfer_from(
            &e,
            &token_a,
            &KeyedAuthorization::Contract,
            &to,
            &Identifier::Contract(pool_id.clone()),
            &amounts.0,
        );

        token::xfer_from(
            &e,
            &token_b,
            &KeyedAuthorization::Contract,
            &to,
            &Identifier::Contract(pool_id.clone()),
            &amounts.1,
        );

        liquidity_pool::deposit(&e, &pool_id, &to);
    }

    fn swap_out(e: Env, to: Identifier, sell: U256, buy: U256, out: BigInt, in_max: BigInt) {
        let (token_a, token_b) = sort(&sell, &buy);
        let pool_id = Self::get_pool(e.clone(), token_a.clone(), token_b);

        let reserves = liquidity_pool::get_rsrvs(&e, &pool_id);

        let reserve_sell: BigInt;
        let reserve_buy: BigInt;
        if sell == token_a {
            reserve_sell = reserves.0;
            reserve_buy = reserves.1;
        } else {
            reserve_sell = reserves.1;
            reserve_buy = reserves.0;
        }

        let n = reserve_sell * out.clone() * BigInt::from_u32(&e, 1000);
        let d = (reserve_buy - out.clone()) * BigInt::from_u32(&e, 997);
        let xfer_amount = (n / d) + BigInt::from_u32(&e, 1);
        if xfer_amount > in_max {
            panic!("in amount is over max")
        }

        token::xfer_from(
            &e,
            &sell,
            &KeyedAuthorization::Contract,
            &to,
            &Identifier::Contract(pool_id.clone()),
            &xfer_amount,
        );

        let out_a: BigInt;
        let out_b: BigInt;
        if sell == token_a {
            out_a = BigInt::from_u32(&e, 0);
            out_b = out;
        } else {
            out_a = out;
            out_b = BigInt::from_u32(&e, 0);
        }

        liquidity_pool::swap(&e, &pool_id, &to, &out_a, &out_b)
    }

    fn sf_withdrw(
        e: Env,
        to: Identifier,
        token_a: U256,
        token_b: U256,
        share_amount: BigInt,
        min_a: BigInt,
        min_b: BigInt,
    ) {
        let pool_id = Self::get_pool(e.clone(), token_a.clone(), token_b);

        let share_token = liquidity_pool::share_id(&e, &pool_id);

        token::xfer_from(
            &e,
            &share_token,
            &KeyedAuthorization::Contract,
            &to,
            &Identifier::Contract(pool_id.clone()),
            &share_amount,
        );

        let (amount_a, amount_b) = liquidity_pool::withdraw(&e, &pool_id, &to);

        if amount_a < min_a || amount_b < min_b {
            panic!("min not satisfied");
        }
    }

    fn get_pool(e: Env, token_a: U256, token_b: U256) -> U256 {
        let salt = pool_salt(&e, &token_a, &token_b);
        get_pool_id(&e, &salt)
    }
}
