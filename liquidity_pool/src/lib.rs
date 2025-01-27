#![no_std]

mod test;

use num_integer::Roots;
use soroban_sdk::{contract, contractimpl, contractmeta, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    TokenA,
    TokenB,
    TotalShares,
    ReserveA,
    ReserveB,
    Shares(Address),
}

fn get_token_a(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::TokenA).unwrap()
}

fn get_token_b(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::TokenB).unwrap()
}

fn get_total_shares(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::TotalShares).unwrap()
}

fn get_reserve_a(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::ReserveA).unwrap()
}

fn get_reserve_b(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::ReserveB).unwrap()
}

fn get_balance(e: &Env, contract: Address) -> i128 {
    token::Client::new(e, &contract).balance(&e.current_contract_address())
}

fn get_balance_a(e: &Env) -> i128 {
    get_balance(e, get_token_a(e))
}

fn get_balance_b(e: &Env) -> i128 {
    get_balance(e, get_token_b(e))
}

fn get_shares(e: &Env, user: &Address) -> i128 {
    e.storage()
        .persistent()
        .get(&DataKey::Shares(user.clone()))
        .unwrap_or(0)
}

fn put_shares(e: &Env, user: &Address, amount: i128) {
    e.storage()
        .persistent()
        .set(&DataKey::Shares(user.clone()), &amount);
}

fn put_token_a(e: &Env, contract: Address) {
    e.storage().instance().set(&DataKey::TokenA, &contract);
}

fn put_token_b(e: &Env, contract: Address) {
    e.storage().instance().set(&DataKey::TokenB, &contract);
}

fn put_total_shares(e: &Env, amount: i128) {
    e.storage().instance().set(&DataKey::TotalShares, &amount)
}

fn put_reserve_a(e: &Env, amount: i128) {
    e.storage().instance().set(&DataKey::ReserveA, &amount)
}

fn put_reserve_b(e: &Env, amount: i128) {
    e.storage().instance().set(&DataKey::ReserveB, &amount)
}

fn burn_shares(e: &Env, from: &Address, amount: i128) {
    let current_shares = get_shares(e, from);
    if current_shares < amount {
        panic!("insufficient shares");
    }
    let total = get_total_shares(e);
    put_shares(e, from, current_shares - amount);
    put_total_shares(e, total - amount);
}

fn mint_shares(e: &Env, to: &Address, amount: i128) {
    let current_shares = get_shares(e, to);
    let total = get_total_shares(e);
    put_shares(e, to, current_shares + amount);
    put_total_shares(e, total + amount);
}

fn transfer(e: &Env, token: Address, to: Address, amount: i128) {
    token::Client::new(e, &token).transfer(&e.current_contract_address(), &to, &amount);
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
        if amount_a > desired_a || amount_a < min_a {
            panic!("amount_a invalid")
        }
        (amount_a, desired_b)
    }
}

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "Constant product AMM with a .3% swap fee"
);

#[contract]
struct LiquidityPool;

#[contractimpl]
impl LiquidityPool {
    pub fn __constructor(e: Env, token_a: Address, token_b: Address) {
        if token_a >= token_b {
            panic!("token_a must be less than token_b");
        }

        put_token_a(&e, token_a);
        put_token_b(&e, token_b);
        put_total_shares(&e, 0);
        put_reserve_a(&e, 0);
        put_reserve_b(&e, 0);
    }

    pub fn balance_shares(e: Env, user: Address) -> i128 {
        get_shares(&e, &user)
    }

    pub fn deposit(
        e: Env,
        to: Address,
        desired_a: i128,
        min_a: i128,
        desired_b: i128,
        min_b: i128,
    ) {
        // Depositor needs to authorize the deposit
        to.require_auth();

        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));

        // Calculate deposit amounts
        let (amount_a, amount_b) =
            get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserve_a, reserve_b);

        if amount_a <= 0 || amount_b <= 0 {
            // If one of the amounts can be zero, we can get into a situation
            // where one of the reserves is 0, which leads to a divide by zero.
            panic!("both amounts must be strictly positive");
        }

        let token_a_client = token::Client::new(&e, &get_token_a(&e));
        let token_b_client = token::Client::new(&e, &get_token_b(&e));

        token_a_client.transfer(&to, &e.current_contract_address(), &amount_a);
        token_b_client.transfer(&to, &e.current_contract_address(), &amount_b);

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

        mint_shares(&e, &to, new_total_shares - total_shares);
        put_reserve_a(&e, balance_a);
        put_reserve_b(&e, balance_b);
    }

    // If "buy_a" is true, the swap will buy token_a and sell token_b. This is flipped if "buy_a" is false.
    // "out" is the amount being bought, with in_max being a safety to make sure you receive at least that amount.
    // swap will transfer the selling token "to" to this contract, and then the contract will transfer the buying token to "to".
    pub fn swap(e: Env, to: Address, buy_a: bool, out: i128, in_max: i128) {
        to.require_auth();

        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));
        let (reserve_sell, reserve_buy) = if buy_a {
            (reserve_b, reserve_a)
        } else {
            (reserve_a, reserve_b)
        };

        if reserve_buy < out {
            panic!("not enough token to buy");
        }

        // First calculate how much needs to be sold to buy amount out from the pool
        let n = reserve_sell * out * 1000;
        let d = (reserve_buy - out) * 997;
        let sell_amount = (n / d) + 1;
        if sell_amount > in_max {
            panic!("in amount is over max")
        }

        // Transfer the amount being sold to the contract
        let sell_token = if buy_a {
            get_token_b(&e)
        } else {
            get_token_a(&e)
        };
        let sell_token_client = token::Client::new(&e, &sell_token);
        sell_token_client.transfer(&to, &e.current_contract_address(), &sell_amount);

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

        let new_reserve_a = balance_a - out_a;
        let new_reserve_b = balance_b - out_b;

        if new_reserve_a <= 0 || new_reserve_b <= 0 {
            panic!("new reserves must be strictly positive");
        }

        put_reserve_a(&e, new_reserve_a);
        put_reserve_b(&e, new_reserve_b);
    }

    // transfers share_amount of pool share tokens to this contract, burns all pools share tokens in this contracts, and sends the
    // corresponding amount of token_a and token_b to "to".
    // Returns amount of both tokens withdrawn
    pub fn withdraw(
        e: Env,
        to: Address,
        share_amount: i128,
        min_a: i128,
        min_b: i128,
    ) -> (i128, i128) {
        to.require_auth();

        let current_shares = get_shares(&e, &to);
        if current_shares < share_amount {
            panic!("insufficient shares");
        }

        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));
        let total_shares = get_total_shares(&e);

        // Calculate withdrawal amounts
        let out_a = (balance_a * share_amount) / total_shares;
        let out_b = (balance_b * share_amount) / total_shares;

        if out_a < min_a || out_b < min_b {
            panic!("min not satisfied");
        }

        burn_shares(&e, &to, share_amount);
        transfer_a(&e, to.clone(), out_a);
        transfer_b(&e, to, out_b);
        put_reserve_a(&e, balance_a - out_a);
        put_reserve_b(&e, balance_b - out_b);

        (out_a, out_b)
    }

    pub fn get_rsrvs(e: Env) -> (i128, i128) {
        (get_reserve_a(&e), get_reserve_b(&e))
    }
}
