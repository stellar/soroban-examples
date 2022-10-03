#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod pool_contract;
mod test;
pub mod testutils;
mod token_contract;

use pool_contract::{create_contract, LiquidityPoolClient};
use token_contract::TokenClient;

use soroban_auth::verify;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, BigInt, Bytes, BytesN, Env, Symbol};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Pool(BytesN<32>),
    Nonce(Identifier),
}

fn get_pool_id(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    e.data().get_unchecked(DataKey::Pool(salt.clone())).unwrap()
}

fn put_pool(e: &Env, salt: &BytesN<32>, pool: &BytesN<32>) {
    e.data().set(DataKey::Pool(salt.clone()), pool.clone())
}

fn has_pool(e: &Env, salt: &BytesN<32>) -> bool {
    e.data().has(DataKey::Pool(salt.clone()))
}

pub trait LiquidityPoolRouterTrait {
    fn sf_deposit(
        e: Env,
        to: Signature,
        nonce: BigInt,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        desired_a: BigInt,
        min_a: BigInt,
        desired_b: BigInt,
        min_b: BigInt,
    );

    // swaps out an exact amount of "buy", in exchange for "sell" that this contract has an
    // allowance for from "to". "sell" amount swapped in must not be greater than "in_max"
    fn swap_out(
        e: Env,
        to: Signature,
        nonce: BigInt,
        sell: BytesN<32>,
        buy: BytesN<32>,
        out: BigInt,
        in_max: BigInt,
    );

    fn sf_withdrw(
        e: Env,
        to: Signature,
        nonce: BigInt,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        share_amount: BigInt,
        min_a: BigInt,
        min_b: BigInt,
    );

    // returns the contract address for the specified token_a/token_b combo
    fn get_pool(e: Env, token_a: BytesN<32>, token_b: BytesN<32>) -> BytesN<32>;

    // Returns the current nonce for "id"
    fn nonce(e: Env, id: Identifier) -> BigInt;
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

    let mut salt_bin = Bytes::new(&e);
    salt_bin.append(&token_a.clone().into());
    salt_bin.append(&token_b.clone().into());
    e.compute_hash_sha256(&salt_bin)
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

struct LiquidityPoolRouter;

#[contractimpl]
impl LiquidityPoolRouterTrait for LiquidityPoolRouter {
    fn sf_deposit(
        e: Env,
        to: Signature,
        nonce: BigInt,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        desired_a: BigInt,
        min_a: BigInt,
        desired_b: BigInt,
        min_b: BigInt,
    ) {
        let to_id = to.identifier(&e);

        verify_and_consume_nonce(&e, &to_id, &nonce);

        verify(
            &e,
            &to,
            Symbol::from_str("sf_deposit"),
            (
                &to_id, nonce, &token_a, &token_b, &desired_a, &min_a, &desired_b, &min_b,
            ),
        );

        let salt = pool_salt(&e, &token_a, &token_b);
        if !has_pool(&e, &salt) {
            let pool_contract_id = create_contract(&e, &salt);

            put_pool(&e, &salt, &pool_contract_id);

            LiquidityPoolClient::new(&e, &pool_contract_id).initialize(&token_a, &token_b);
        }

        let pool_id = get_pool_id(&e, &salt);

        let reserves = LiquidityPoolClient::new(&e, &pool_id).get_rsrvs();
        let amounts = get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserves);

        let client_a = TokenClient::new(&e, token_a);
        client_a.xfer_from(
            &Signature::Invoker,
            &BigInt::zero(&e),
            &to_id,
            &Identifier::Contract(pool_id.clone()),
            &amounts.0,
        );

        let client_b = TokenClient::new(&e, token_b);
        client_b.xfer_from(
            &Signature::Invoker,
            &BigInt::zero(&e),
            &to_id,
            &Identifier::Contract(pool_id.clone()),
            &amounts.1,
        );

        LiquidityPoolClient::new(&e, &pool_id).deposit(&to_id);
    }

    fn swap_out(
        e: Env,
        to: Signature,
        nonce: BigInt,
        sell: BytesN<32>,
        buy: BytesN<32>,
        out: BigInt,
        in_max: BigInt,
    ) {
        let to_id = to.identifier(&e);

        verify_and_consume_nonce(&e, &to_id, &nonce);

        verify(
            &e,
            &to,
            Symbol::from_str("swap_out"),
            (&to_id, nonce, &sell, &buy, &out, &in_max),
        );

        let (token_a, token_b) = sort(&sell, &buy);
        let pool_id = Self::get_pool(e.clone(), token_a.clone(), token_b);

        let reserves = LiquidityPoolClient::new(&e, &pool_id).get_rsrvs();

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

        let client = TokenClient::new(&e, &sell);
        client.xfer_from(
            &Signature::Invoker,
            &BigInt::zero(&e),
            &to_id,
            &Identifier::Contract(pool_id.clone()),
            &xfer_amount,
        );

        let out_a: BigInt;
        let out_b: BigInt;
        if sell == token_a {
            out_a = BigInt::zero(&e);
            out_b = out;
        } else {
            out_a = out;
            out_b = BigInt::zero(&e);
        }

        LiquidityPoolClient::new(&e, &pool_id).swap(&to_id, &out_a, &out_b)
    }

    fn sf_withdrw(
        e: Env,
        to: Signature,
        nonce: BigInt,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        share_amount: BigInt,
        min_a: BigInt,
        min_b: BigInt,
    ) {
        let to_id = to.identifier(&e);

        verify_and_consume_nonce(&e, &to_id, &nonce);

        verify(
            &e,
            &to,
            Symbol::from_str("sf_withdrw"),
            (
                &to_id,
                nonce,
                &token_a,
                &token_b,
                &share_amount,
                &min_a,
                &min_b,
            ),
        );

        let pool_id = Self::get_pool(e.clone(), token_a.clone(), token_b);

        let pool_client = LiquidityPoolClient::new(&e, &pool_id);
        let share_token = pool_client.share_id();

        let client = TokenClient::new(&e, &share_token);
        client.xfer_from(
            &Signature::Invoker,
            &BigInt::zero(&e),
            &to_id,
            &Identifier::Contract(pool_id.clone()),
            &share_amount,
        );

        let (amount_a, amount_b) = pool_client.withdraw(&to_id);

        if amount_a < min_a || amount_b < min_b {
            panic!("min not satisfied");
        }
    }

    fn get_pool(e: Env, token_a: BytesN<32>, token_b: BytesN<32>) -> BytesN<32> {
        let salt = pool_salt(&e, &token_a, &token_b);
        get_pool_id(&e, &salt)
    }

    fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, &id)
    }
}
