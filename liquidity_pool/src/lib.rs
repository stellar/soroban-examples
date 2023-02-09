#![no_std]

mod test;
mod token;

use num_integer::Roots;
use soroban_sdk::{contractimpl, Address, Bytes, BytesN, ConversionError, Env, RawVal, TryFromVal};
use token::create_contract;

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

impl TryFromVal<Env, DataKey> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

fn get_token_a(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(&DataKey::TokenA).unwrap()
}

fn get_token_b(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(&DataKey::TokenB).unwrap()
}

fn get_token_share(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(&DataKey::TokenShare).unwrap()
}

fn get_total_shares(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::TotalShares).unwrap()
}

fn get_reserve_a(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::ReserveA).unwrap()
}

fn get_reserve_b(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::ReserveB).unwrap()
}

fn get_balance(e: &Env, contract_id: BytesN<32>) -> i128 {
    token::Client::new(e, &contract_id).balance(&e.current_contract_address())
}

fn get_balance_a(e: &Env) -> i128 {
    get_balance(e, get_token_a(e))
}

fn get_balance_b(e: &Env) -> i128 {
    get_balance(e, get_token_b(e))
}

fn get_balance_shares(e: &Env) -> i128 {
    get_balance(e, get_token_share(e))
}

fn put_token_a(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(&DataKey::TokenA, &contract_id);
}

fn put_token_b(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(&DataKey::TokenB, &contract_id);
}

fn put_token_share(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(&DataKey::TokenShare, &contract_id);
}

fn put_total_shares(e: &Env, amount: i128) {
    e.storage().set(&DataKey::TotalShares, &amount)
}

fn put_reserve_a(e: &Env, amount: i128) {
    e.storage().set(&DataKey::ReserveA, &amount)
}

fn put_reserve_b(e: &Env, amount: i128) {
    e.storage().set(&DataKey::ReserveB, &amount)
}

fn burn_shares(e: &Env, amount: i128) {
    let total = get_total_shares(e);
    let share_contract_id = get_token_share(e);

    token::Client::new(e, &share_contract_id).burn(&e.current_contract_address(), &amount);
    put_total_shares(e, total - amount);
}

fn mint_shares(e: &Env, to: Address, amount: i128) {
    let total = get_total_shares(e);
    let share_contract_id = get_token_share(e);

    token::Client::new(e, &share_contract_id).mint(&e.current_contract_address(), &to, &amount);

    put_total_shares(e, total + amount);
}

fn transfer(e: &Env, contract_id: BytesN<32>, to: Address, amount: i128) {
    token::Client::new(e, &contract_id).xfer(&e.current_contract_address(), &to, &amount);
}

fn transfer_a(e: &Env, to: Address, amount: i128) {
    transfer(e, get_token_a(e), to, amount);
}

fn transfer_b(e: &Env, to: Address, amount: i128) {
    transfer(e, get_token_b(e), to, amount);
}

fn get_deposit_amounts(
    desired_a: i128,
    min_a: i128,
    desired_b: i128,
    min_b: i128,
    reserve_a: i128,
    reserve_b: i128,
) -> (i128, i128) {
    if reserve_a == 0 && reserve_b == 0 {
        return (desired_a, desired_b);
    }

    let amount_b = desired_a * reserve_b / reserve_a;
    if amount_b <= desired_b {
        if amount_b < min_b {
            panic!("amount_b less than min")
        }
        (desired_a, amount_b)
    } else {
        let amount_a = desired_b * reserve_a / reserve_b;
        if amount_a > desired_a || desired_a < min_a {
            panic!("amount_a invalid")
        }
        (amount_a, desired_b)
    }
}

pub trait LiquidityPoolTrait {
    // Sets the token contract addresses for this pool
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_a: BytesN<32>, token_b: BytesN<32>);

    // Returns the token contract address for the pool share token
    fn share_id(e: Env) -> BytesN<32>;

    // Deposits token_a and token_b. Also mints pool shares for the "to" Identifier. The amount minted
    // is determined based on the difference between the reserves stored by this contract, and
    // the actual balance of token_a and token_b for this contract.
    fn deposit(e: Env, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128);

    // If "buy_a" is true, the swap will buy token_a and sell token_b. This is flipped if "buy_a" is false.
    // "out" is the amount being bought, with in_max being a safety to make sure you receive at least that amount.
    // swap will xfer the selling token "to" to this contract, and then the contract will xfer the buying token to "to".
    fn swap(e: Env, to: Address, buy_a: bool, out: i128, in_max: i128);

    // xfers share_amount of pool share tokens to this contract, burns all pools share tokens in this contracts, and sends the
    // corresponding amount of token_a and token_b to "to".
    // Returns amount of both tokens withdrawn
    fn withdraw(e: Env, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128);

    fn get_rsrvs(e: Env) -> (i128, i128);
}

struct LiquidityPool;

#[contractimpl]
impl LiquidityPoolTrait for LiquidityPool {
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_a: BytesN<32>, token_b: BytesN<32>) {
        if token_a >= token_b {
            panic!("token_a must be less than token_b");
        }

        let share_contract_id = create_contract(&e, &token_wasm_hash, &token_a, &token_b);
        token::Client::new(&e, &share_contract_id).initialize(
            &e.current_contract_address(),
            &7u32,
            &Bytes::from_slice(&e, b"Pool Share Token"),
            &Bytes::from_slice(&e, b"POOL"),
        );

        put_token_a(&e, token_a);
        put_token_b(&e, token_b);
        put_token_share(&e, share_contract_id.try_into().unwrap());
        put_total_shares(&e, 0);
        put_reserve_a(&e, 0);
        put_reserve_b(&e, 0);
    }

    fn share_id(e: Env) -> BytesN<32> {
        get_token_share(&e)
    }

    fn deposit(e: Env, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128) {
        // Depositor needs to authorize the deposit
        to.require_auth();

        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));

        // Calculate deposit amounts
        let amounts = get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserve_a, reserve_b);

        let token_a_client = token::Client::new(&e, &get_token_a(&e));
        let token_b_client = token::Client::new(&e, &get_token_b(&e));

        token_a_client.xfer(&to, &e.current_contract_address(), &amounts.0);
        token_b_client.xfer(&to, &e.current_contract_address(), &amounts.1);

        // Now calculate how many new pool shares to mint
        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));
        let total_shares = get_total_shares(&e);

        let zero = 0;
        let new_total_shares = if reserve_a > zero && reserve_b > zero {
            let shares_a = (balance_a * total_shares) / reserve_a;
            let shares_b = (balance_b * total_shares) / reserve_b;
            shares_a.min(shares_b)
        } else {
            (balance_a * balance_b).sqrt()
        };

        mint_shares(&e, to, new_total_shares - total_shares);
        put_reserve_a(&e, balance_a);
        put_reserve_b(&e, balance_b);
    }

    fn swap(e: Env, to: Address, buy_a: bool, out: i128, in_max: i128) {
        to.require_auth();

        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));
        let (reserve_sell, reserve_buy) = if buy_a {
            (reserve_b, reserve_a)
        } else {
            (reserve_a, reserve_b)
        };

        // First calculate how much needs to be sold to buy amount out from the pool
        let n = reserve_sell * out * 1000;
        let d = (reserve_buy - out) * 997;
        let sell_amount = (n / d) + 1;
        if sell_amount > in_max {
            panic!("in amount is over max")
        }

        // Xfer the amount being sold to the contract
        let sell_token = if buy_a {
            get_token_b(&e)
        } else {
            get_token_a(&e)
        };
        let sell_token_client = token::Client::new(&e, &sell_token);
        sell_token_client.xfer(&to, &e.current_contract_address(), &sell_amount);

        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));

        // residue_numerator and residue_denominator are the amount that the invariant considers after
        // deducting the fee, scaled up by 1000 to avoid fractions
        let residue_numerator = 997;
        let residue_denominator = 1000;
        let zero = 0;

        let new_invariant_factor = |balance: i128, reserve: i128, out: i128| {
            let delta = balance - reserve - out;
            let adj_delta = if delta > zero {
                residue_numerator * delta
            } else {
                residue_denominator * delta
            };
            residue_denominator * reserve + adj_delta
        };

        let (out_a, out_b) = if buy_a { (out, 0) } else { (0, out) };

        let new_inv_a = new_invariant_factor(balance_a, reserve_a, out_a);
        let new_inv_b = new_invariant_factor(balance_b, reserve_b, out_b);
        let old_inv_a = residue_denominator * reserve_a;
        let old_inv_b = residue_denominator * reserve_b;

        if new_inv_a * new_inv_b < old_inv_a * old_inv_b {
            panic!("constant product invariant does not hold");
        }

        if buy_a {
            transfer_a(&e, to, out_a);
        } else {
            transfer_b(&e, to, out_b);
        }

        put_reserve_a(&e, balance_a - out_a);
        put_reserve_b(&e, balance_b - out_b);
    }

    fn withdraw(e: Env, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128) {
        to.require_auth();

        // First transfer the pool shares that need to be redeemed
        let share_token_client = token::Client::new(&e, &get_token_share(&e));
        share_token_client.xfer(&to, &e.current_contract_address(), &share_amount);

        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));
        let balance_shares = get_balance_shares(&e);

        let total_shares = get_total_shares(&e);

        // Now calculate the withdraw amounts
        let out_a = (balance_a * balance_shares) / total_shares;
        let out_b = (balance_b * balance_shares) / total_shares;

        if out_a < min_a || out_b < min_b {
            panic!("min not satisfied");
        }

        burn_shares(&e, balance_shares);
        transfer_a(&e, to.clone(), out_a);
        transfer_b(&e, to, out_b);
        put_reserve_a(&e, balance_a - out_a);
        put_reserve_b(&e, balance_b - out_b);

        (out_a, out_b)
    }

    fn get_rsrvs(e: Env) -> (i128, i128) {
        (get_reserve_a(&e), get_reserve_b(&e))
    }
}
