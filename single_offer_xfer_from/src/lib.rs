#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod test;
pub mod testutils;
mod token_contract;

use token_contract::TokenClient;

use soroban_auth::{
    verify, {Identifier, Signature},
};
use soroban_sdk::{contractimpl, contracttype, BigInt, BytesN, Env, Symbol};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    SellToken,
    BuyToken,
    Admin,
    Price,
    Nonce(Identifier),
}

// Price is 1 unit of selling in terms of buying. For example, if you wanted
// to sell 30 XLM and buy 5 BTC, the price would be Price{n: 5, d: 30}.
#[derive(Clone)]
#[contracttype]
pub struct Price {
    pub n: u32,
    pub d: u32,
}

fn get_sell_token(e: &Env) -> BytesN<32> {
    e.data().get_unchecked(DataKey::SellToken).unwrap()
}

fn get_buy_token(e: &Env) -> BytesN<32> {
    e.data().get_unchecked(DataKey::BuyToken).unwrap()
}

fn put_sell_token(e: &Env, contract_id: BytesN<32>) {
    e.data().set(DataKey::SellToken, contract_id);
}

fn put_buy_token(e: &Env, contract_id: BytesN<32>) {
    e.data().set(DataKey::BuyToken, contract_id);
}

fn put_price(e: &Env, price: Price) {
    e.data().set(DataKey::Price, price);
}

fn get_price(e: &Env) -> Price {
    e.data().get_unchecked(DataKey::Price).unwrap()
}

fn transfer_from(
    e: &Env,
    contract_id: BytesN<32>,
    from: &Identifier,
    to: &Identifier,
    amount: &BigInt,
) {
    let client = TokenClient::new(&e, &contract_id);
    client.xfer_from(&Signature::Invoker, &BigInt::zero(&e), &from, &to, &amount)
}

fn transfer_sell(e: &Env, from: &Identifier, to: &Identifier, amount: &BigInt) {
    transfer_from(&e, get_sell_token(&e), from, to, amount);
}

fn transfer_buy(e: &Env, from: &Identifier, to: &Identifier, amount: &BigInt) {
    transfer_from(&e, get_buy_token(&e), from, to, amount);
}

fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.data().has(key)
}

fn read_administrator(e: &Env) -> Identifier {
    let key = DataKey::Admin;
    e.data().get_unchecked(key).unwrap()
}

fn write_administrator(e: &Env, id: Identifier) {
    let key = DataKey::Admin;
    e.data().set(key, id);
}

pub fn check_admin(e: &Env, auth: &Signature) {
    let auth_id = auth.identifier(&e);
    if auth_id != read_administrator(&e) {
        panic!("not authorized by admin")
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

/*
How to use this contract to trade

1. call initialize(seller, USDC_ADDR, BTC_ADDR, 1, 10). Seller is now the admin
2. seller calls USDC.approve(seller_auth, offer_contract, 10)
3. buyer calls BTC.approve(buyer_auth, offer_contract, 1)
4. buyer calls trade(buyer_auth, 1, 10). This contract will send 1 BTC to
   seller and 10 USDC to buyer.
*/

pub trait SingleOfferXferFromTrait {
    // See comment above the Price struct for information on pricing
    fn initialize(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
        n: u32,
        d: u32,
    );

    // Returns the current nonce for "id"
    fn nonce(e: Env, id: Identifier) -> BigInt;

    // Sends amount_to_sell of buy_token to the admin, and sends amount_to_sell * d / n of
    // sell_token to "to". Allowances must be sufficient for this contract address to send
    // sell_token from admin and buy_token from "to". Needs to be authorized by "to".
    // (Signature is required because a different entity
    // could submit the trade with a bad min, or the admin could change the price and then
    // call trade).
    fn trade(e: Env, to: Signature, nonce: BigInt, amount_to_sell: BigInt, min: BigInt);

    // Updates the price. Must be authorized by admin
    fn updt_price(e: Env, admin: Signature, nonce: BigInt, n: u32, d: u32);

    // Get the current price
    fn get_price(e: Env) -> Price;
}

struct SingleOfferXferFrom;

#[contractimpl]
impl SingleOfferXferFromTrait for SingleOfferXferFrom {
    fn initialize(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
        n: u32,
        d: u32,
    ) {
        if has_administrator(&e) {
            panic!("admin is already set");
        }

        if d == 0 {
            panic!("d is zero but cannot be zero");
        }

        write_administrator(&e, admin);

        put_sell_token(&e, sell_token);
        put_buy_token(&e, buy_token);
        put_price(&e, Price { n, d });
    }

    fn trade(e: Env, to: Signature, nonce: BigInt, amount_to_sell: BigInt, min: BigInt) {
        let to_id = to.identifier(&e);

        verify_and_consume_nonce(&e, &to_id, &nonce);

        verify(
            &e,
            &to,
            Symbol::from_str("trade"),
            (&to_id, nonce, &amount_to_sell, &min),
        );

        let price = get_price(&e);

        let amount =
            amount_to_sell.clone() * BigInt::from_u32(&e, price.d) / BigInt::from_u32(&e, price.n);

        if amount < min {
            panic!("will receive less than min");
        }

        let admin = read_administrator(&e);

        transfer_sell(&e, &admin, &to_id, &amount);
        transfer_buy(&e, &to_id, &admin, &amount_to_sell);
    }

    fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, &id)
    }

    fn updt_price(e: Env, admin: Signature, nonce: BigInt, n: u32, d: u32) {
        check_admin(&e, &admin);
        let admin_id = admin.identifier(&e);

        if d == 0 {
            panic!("d is zero but cannot be zero")
        }

        verify_and_consume_nonce(&e, &admin_id, &nonce);

        verify(
            &e,
            &admin,
            Symbol::from_str("updt_price"),
            (admin_id, nonce, &n, &d),
        );

        put_price(&e, Price { n, d });
    }

    fn get_price(e: Env) -> Price {
        get_price(&e)
    }
}
