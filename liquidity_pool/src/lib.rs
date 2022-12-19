#![no_std]

mod test;
pub mod testutils;
mod token;

use num_integer::Roots;
use soroban_sdk::{contractimpl, Bytes, BytesN, Env, IntoVal, RawVal};
use token::create_contract;
use token::{Identifier, Signature};

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

impl IntoVal<Env, RawVal> for DataKey {
    fn into_val(self, env: &Env) -> RawVal {
        (self as u32).into_val(env)
    }
}

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract())
}

fn get_token_a(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::TokenA).unwrap()
}

fn get_token_b(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::TokenB).unwrap()
}

fn get_token_share(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::TokenShare).unwrap()
}

fn get_total_shares(e: &Env) -> i128 {
    e.storage().get_unchecked(DataKey::TotalShares).unwrap()
}

fn get_reserve_a(e: &Env) -> i128 {
    e.storage().get_unchecked(DataKey::ReserveA).unwrap()
}

fn get_reserve_b(e: &Env) -> i128 {
    e.storage().get_unchecked(DataKey::ReserveB).unwrap()
}

fn get_balance(e: &Env, contract_id: BytesN<32>) -> i128 {
    token::Client::new(e, contract_id).balance(&get_contract_id(e))
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
    e.storage().set(DataKey::TokenA, contract_id);
}

fn put_token_b(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::TokenB, contract_id);
}

fn put_token_share(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::TokenShare, contract_id);
}

fn put_total_shares(e: &Env, amount: i128) {
    e.storage().set(DataKey::TotalShares, amount)
}

fn put_reserve_a(e: &Env, amount: i128) {
    e.storage().set(DataKey::ReserveA, amount)
}

fn put_reserve_b(e: &Env, amount: i128) {
    e.storage().set(DataKey::ReserveB, amount)
}

fn burn_shares(e: &Env, amount: i128) {
    let total = get_total_shares(e);
    let share_contract_id = get_token_share(e);

    token::Client::new(e, share_contract_id).burn(
        &Signature::Invoker,
        &0,
        &get_contract_id(e),
        &amount,
    );
    put_total_shares(e, total - amount);
}

fn mint_shares(e: &Env, to: Identifier, amount: i128) {
    let total = get_total_shares(e);
    let share_contract_id = get_token_share(e);

    token::Client::new(e, share_contract_id).mint(&Signature::Invoker, &0, &to, &amount);

    put_total_shares(e, total + amount);
}

fn transfer(e: &Env, contract_id: BytesN<32>, to: Identifier, amount: i128) {
    token::Client::new(e, contract_id).xfer(&Signature::Invoker, &0, &to, &amount);
}

fn transfer_a(e: &Env, to: Identifier, amount: i128) {
    transfer(e, get_token_a(e), to, amount);
}

fn transfer_b(e: &Env, to: Identifier, amount: i128) {
    transfer(e, get_token_b(e), to, amount);
}

/*
How to use this contract to swap

1. call initialize(provider, USDC_ADDR, BTC_ADDR).
2. provider sends 100 USDC and 100 BTC to this contracts address and calls deposit(provider) in the same transaction.
   provider now has 100 pool share tokens, and this contract has 100 USDC and 100 BTC.
3. swapper sends 100 USDC to this contract and calls swap(swapper, 0, 49) in the same transaction. 49 BTC will be sent to swapper.
4. provider sends 100 pool share tokens to this contract, and then calls withdraw(provider). 51 BTC and 200 USDC will be sent to
   provider, and the 100 pool share tokens in this contract will be burned.
*/
pub trait LiquidityPoolTrait {
    // Sets the token contract addresses for this pool
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_a: BytesN<32>, token_b: BytesN<32>);

    // Returns the token contract address for the pool share token
    fn share_id(e: Env) -> BytesN<32>;

    // Mints pool shares for the "to" Identifier. The amount minted is determined based on the difference
    // between the reserves stored by this contract, and the actual balance of token_a and token_b for this
    // contract. This means that an account calling deposit must first send token_a and token_b to this contract,
    // and them call deposit in the same transaction. If these steps aren't done atomically, then the depositer
    // could lose their tokens.
    fn deposit(e: Env, to: Identifier);

    // Does a swap and sends out_a of token_a and out_b of token_b to the "to" Identifier if the constant product invariant still
    // holds. The difference between the balance and reserve for each token in this contract determines the amounts that
    // can be swapped. For this to be used safely, the swapper must send tokens to this contract and call swap in the
    // same transaction. If these steps aren't done atomically, then the swapper could lose their tokens.
    fn swap(e: Env, to: Identifier, out_a: i128, out_b: i128);

    // Burns all pool share tokens in this contract, and sends the corresponding amount of token_a and token_b to
    // "to". For this to be used safely, the withdrawer must send the pool share token to this contract and call
    // withdraw in the same transaction. If these steps aren't done atomically, then the withdrawer
    // could lose their tokens.
    // Returns amount of both tokens withdrawn
    fn withdraw(e: Env, to: Identifier) -> (i128, i128);

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
        token::Client::new(&e, share_contract_id.clone()).initialize(
            &get_contract_id(&e),
            &7u32,
            &Bytes::from_slice(&e, b"name"),
            &Bytes::from_slice(&e, b"symbol"),
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

    fn deposit(e: Env, to: Identifier) {
        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));
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

    fn swap(e: Env, to: Identifier, out_a: i128, out_b: i128) {
        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));
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
        let new_inv_a = new_invariant_factor(balance_a, reserve_a, out_a);
        let new_inv_b = new_invariant_factor(balance_b, reserve_b, out_b);
        let old_inv_a = residue_denominator * reserve_a;
        let old_inv_b = residue_denominator * reserve_b;

        if new_inv_a * new_inv_b < old_inv_a * old_inv_b {
            panic!("constant product invariant does not hold");
        }

        transfer_a(&e, to.clone(), out_a);
        transfer_b(&e, to, out_b);
        put_reserve_a(&e, balance_a - out_a);
        put_reserve_b(&e, balance_b - out_b);
    }

    fn withdraw(e: Env, to: Identifier) -> (i128, i128) {
        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));
        let balance_shares = get_balance_shares(&e);
        let total_shares = get_total_shares(&e);

        let out_a = (balance_a * balance_shares) / total_shares;
        let out_b = (balance_b * balance_shares) / total_shares;

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
