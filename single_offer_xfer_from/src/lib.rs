#![no_std]

mod test;
pub mod testutils;

use soroban_sdk::{contractimpl, contracttype, BytesN, Env};
use token::{Identifier, Signature};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    SellToken,
    BuyToken,
    Admin,
    Price,
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
    amount: &i128,
) {
    let client = token::Client::new(e, &contract_id);
    client.xfer_from(&Signature::Invoker, &0, from, to, amount)
}

fn transfer_sell(e: &Env, from: &Identifier, to: &Identifier, amount: &i128) {
    transfer_from(e, get_sell_token(e), from, to, amount);
}

fn transfer_buy(e: &Env, from: &Identifier, to: &Identifier, amount: &i128) {
    transfer_from(e, get_buy_token(e), from, to, amount);
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

pub fn check_admin(e: &Env, auth_id: Identifier) {
    if auth_id != read_administrator(e) {
        panic!("not authorized by admin")
    }
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

    // Sends amount_to_sell of buy_token to the admin, and sends amount_to_sell * d / n of
    // sell_token to "to". Allowances must be sufficient for this contract address to send
    // sell_token from admin and buy_token from the invoker.
    fn trade(e: Env, amount_to_sell: i128, min: i128);

    // Updates the price. Must be authorized by admin
    fn updt_price(e: Env, n: u32, d: u32);

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

    fn trade(e: Env, amount_to_sell: i128, min: i128) {
        let price = get_price(&e);

        let amount = amount_to_sell * price.d as i128 / price.n as i128;

        if amount < min {
            panic!("will receive less than min");
        }

        let admin = read_administrator(&e);

        let invoker = e.invoker().into();
        transfer_sell(&e, &admin, &invoker, &amount);
        transfer_buy(&e, &invoker, &admin, &amount_to_sell);
    }

    fn updt_price(e: Env, n: u32, d: u32) {
        check_admin(&e, e.invoker().into());

        put_price(&e, Price { n, d });
    }

    fn get_price(e: Env) -> Price {
        get_price(&e)
    }
}
