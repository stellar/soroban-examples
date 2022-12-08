#![no_std]

mod pool_contract;
mod test;
pub mod testutils;

use pool_contract::LiquidityPoolClient;
use soroban_sdk::{contractimpl, contracttype, Bytes, BytesN, Env};
use token::{Identifier, Signature};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Pool(BytesN<32>),
}

fn get_pool_id(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    e.storage()
        .get_unchecked(DataKey::Pool(salt.clone()))
        .unwrap()
}

fn put_pool(e: &Env, salt: &BytesN<32>, pool: &BytesN<32>) {
    e.storage().set(DataKey::Pool(salt.clone()), pool.clone())
}

fn has_pool(e: &Env, salt: &BytesN<32>) -> bool {
    e.storage().has(DataKey::Pool(salt.clone()))
}

pub trait LiquidityPoolRouterTrait {
    fn sf_deposit(
        e: Env,
        liqiudity_pool_wasm_hash: BytesN<32>,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        desired_a: i128,
        min_a: i128,
        desired_b: i128,
        min_b: i128,
    );

    // swaps out an exact amount of "buy", in exchange for "sell" that this contract has an
    // allowance for from "to". "sell" amount swapped in must not be greater than "in_max"
    fn swap_out(e: Env, sell: BytesN<32>, buy: BytesN<32>, out: i128, in_max: i128);

    fn sf_withdrw(
        e: Env,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        share_amount: i128,
        min_a: i128,
        min_b: i128,
    );

    // returns the contract address for the specified token_a/token_b combo
    fn get_pool(e: Env, token_a: BytesN<32>, token_b: BytesN<32>) -> BytesN<32>;
}

fn sort(a: &BytesN<32>, b: &BytesN<32>) -> (BytesN<32>, BytesN<32>) {
    if a < b {
        return (a.clone(), b.clone());
    } else if a > b {
        return (b.clone(), a.clone());
    }
    panic!("a and b can't be the same")
}

pub fn pool_salt(e: &Env, token_a: &BytesN<32>, token_b: &BytesN<32>) -> BytesN<32> {
    if token_a >= token_b {
        panic!("token_a must be less t&han token_b");
    }

    let mut salt_bin = Bytes::new(e);
    salt_bin.append(&token_a.clone().into());
    salt_bin.append(&token_b.clone().into());
    e.crypto().sha256(&salt_bin)
}

fn get_deposit_amounts(
    desired_a: i128,
    min_a: i128,
    desired_b: i128,
    min_b: i128,
    reserves: (i128, i128),
) -> (i128, i128) {
    if reserves.0 == 0 && reserves.1 == 0 {
        return (desired_a, desired_b);
    }

    let amount_b = desired_a * reserves.1 / reserves.0;
    if amount_b <= desired_b {
        if amount_b < min_b {
            panic!("amount_b less than min")
        }
        (desired_a, amount_b)
    } else {
        let amount_a = desired_b * reserves.0 / reserves.1;
        if amount_a > desired_a || desired_a < min_a {
            panic!("amount_a invalid")
        }
        (amount_a, desired_b)
    }
}

struct LiquidityPoolRouter;

#[contractimpl]
impl LiquidityPoolRouterTrait for LiquidityPoolRouter {
    fn sf_deposit(
        e: Env,
        liquidity_pool_wasm_hash: BytesN<32>,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        desired_a: i128,
        min_a: i128,
        desired_b: i128,
        min_b: i128,
    ) {
        let salt = pool_salt(&e, &token_a, &token_b);
        if !has_pool(&e, &salt) {
            let pool_contract_id = e
                .deployer()
                .with_current_contract(salt.clone())
                .deploy(liquidity_pool_wasm_hash);

            put_pool(&e, &salt, &pool_contract_id);

            LiquidityPoolClient::new(&e, &pool_contract_id).initialize(&token_a, &token_b);
        }

        let pool_id = get_pool_id(&e, &salt);

        let reserves = LiquidityPoolClient::new(&e, &pool_id).get_rsrvs();
        let amounts = get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserves);

        let invoker = e.invoker();

        let client_a = token::Client::new(&e, token_a);
        client_a.xfer_from(
            &Signature::Invoker,
            &0,
            &invoker.clone().into(),
            &Identifier::Contract(pool_id.clone()),
            &amounts.0,
        );

        let client_b = token::Client::new(&e, token_b);
        client_b.xfer_from(
            &Signature::Invoker,
            &0,
            &invoker.clone().into(),
            &Identifier::Contract(pool_id.clone()),
            &amounts.1,
        );

        LiquidityPoolClient::new(&e, &pool_id).deposit(&invoker.into());
    }

    fn swap_out(e: Env, sell: BytesN<32>, buy: BytesN<32>, out: i128, in_max: i128) {
        let (token_a, token_b) = sort(&sell, &buy);
        let pool_id = Self::get_pool(e.clone(), token_a.clone(), token_b);

        let reserves = LiquidityPoolClient::new(&e, &pool_id).get_rsrvs();

        let reserve_sell: i128;
        let reserve_buy: i128;
        if sell == token_a {
            reserve_sell = reserves.0;
            reserve_buy = reserves.1;
        } else {
            reserve_sell = reserves.1;
            reserve_buy = reserves.0;
        }

        let n = reserve_sell * out * 1000;
        let d = (reserve_buy - out) * 997;
        let xfer_amount = (n / d) + 1;
        if xfer_amount > in_max {
            panic!("in amount is over max")
        }

        let invoker = e.invoker();

        let client = token::Client::new(&e, &sell);
        client.xfer_from(
            &Signature::Invoker,
            &0,
            &invoker.clone().into(),
            &Identifier::Contract(pool_id.clone()),
            &xfer_amount,
        );

        let out_a: i128;
        let out_b: i128;
        if sell == token_a {
            out_a = 0;
            out_b = out;
        } else {
            out_a = out;
            out_b = 0;
        }

        LiquidityPoolClient::new(&e, &pool_id).swap(&invoker.into(), &out_a, &out_b)
    }

    fn sf_withdrw(
        e: Env,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        share_amount: i128,
        min_a: i128,
        min_b: i128,
    ) {
        let pool_id = Self::get_pool(e.clone(), token_a, token_b);

        let pool_client = LiquidityPoolClient::new(&e, &pool_id);
        let share_token = pool_client.share_id();

        let invoker = e.invoker();
        let client = token::Client::new(&e, &share_token);
        client.xfer_from(
            &Signature::Invoker,
            &0,
            &invoker.clone().into(),
            &Identifier::Contract(pool_id),
            &share_amount,
        );

        let (amount_a, amount_b) = pool_client.withdraw(&invoker.into());

        if amount_a < min_a || amount_b < min_b {
            panic!("min not satisfied");
        }
    }

    fn get_pool(e: Env, token_a: BytesN<32>, token_b: BytesN<32>) -> BytesN<32> {
        let salt = pool_salt(&e, &token_a, &token_b);
        get_pool_id(&e, &salt)
    }
}
