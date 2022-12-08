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

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract())
}

fn get_sell_token(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::SellToken).unwrap()
}

fn get_buy_token(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::BuyToken).unwrap()
}

fn get_balance(e: &Env, contract_id: BytesN<32>) -> i128 {
    token::Client::new(e, contract_id).balance(&get_contract_id(e))
}

fn get_balance_buy(e: &Env) -> i128 {
    get_balance(e, get_buy_token(e))
}

fn put_sell_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::SellToken, contract_id);
}

fn put_buy_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::BuyToken, contract_id);
}

fn put_price(e: &Env, price: Price) {
    e.storage().set(DataKey::Price, price);
}

fn load_price(e: &Env) -> Price {
    e.storage().get_unchecked(DataKey::Price).unwrap()
}

fn transfer(e: &Env, contract_id: BytesN<32>, to: Identifier, amount: i128) {
    let client = token::Client::new(e, contract_id);
    client.xfer(&Signature::Invoker, &0, &to, &amount);
}

fn transfer_sell(e: &Env, to: Identifier, amount: i128) {
    transfer(e, get_sell_token(e), to, amount);
}

fn transfer_buy(e: &Env, to: Identifier, amount: i128) {
    transfer(e, get_buy_token(e), to, amount);
}

fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().has(key)
}

fn read_administrator(e: &Env) -> Identifier {
    let key = DataKey::Admin;
    e.storage().get_unchecked(key).unwrap()
}

fn write_administrator(e: &Env, id: Identifier) {
    let key = DataKey::Admin;
    e.storage().set(key, id);
}

pub fn check_admin(e: &Env, auth_id: &Identifier) {
    if *auth_id != read_administrator(e) {
        panic!("not authorized by admin")
    }
}

/*
How to use this contract to trade

1. call initialize(seller, USDC_ADDR, BTC_ADDR, 1, 10). Seller is now the admin
2. seller sends 20 USDC to this contracts address.
3. buyer sends 1 BTC to this contracts address AND calls trade(buyer, 10). This contract will send 1 BTC to
   seller and 10 USDC to buyer. If these two actions are not done atomically, then the 1 BTC sent to this
   address can be taken by another user calling trade.
4. call withdraw(sellerAuth, 10). This will send the remaining 10 USDC in the contract back to seller.
*/
pub trait SingleOfferTrait {
    // See comment above the Price struct for information on pricing
    // Sets the admin, the sell/buy tokens, and the price
    fn initialize(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
        n: u32,
        d: u32,
    );

    // Sends the full balance of this contracts buy_token balance (let's call this BuyB) to the admin, and
    // also sends buyB * d / n of the sell_token to the "to" identifier specified in trade call. Note that
    // the seller and the buyer need to transfer the sell_token and buy_token to this contract prior to calling
    // trade. Due to this and the fact that the buyer is a parameter to trade, the buyer must tranfer the buy_token
    // to the contract and call trade in the same transaction for safety.
    fn trade(e: Env, to: Identifier, min: i128);

    // Sends amount of sell_token from this contract to the admin. Must be authorized by admin
    fn withdraw(e: Env, amount: i128);

    // Updates the price. Must be authorized by admin
    fn updt_price(e: Env, n: u32, d: u32);

    // Get the current price
    fn get_price(e: Env) -> Price;

    fn get_sell(e: Env) -> BytesN<32>;

    fn get_buy(e: Env) -> BytesN<32>;
}

pub struct SingleOffer;

#[contractimpl]
impl SingleOfferTrait for SingleOffer {
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

    fn trade(e: Env, to: Identifier, min: i128) {
        let balance_buy_token = get_balance_buy(&e);

        let price = load_price(&e);

        let amount = balance_buy_token * price.d as i128 / price.n as i128;

        if amount < min {
            panic!("will receive less than min");
        }

        transfer_sell(&e, to, amount);

        let admin = read_administrator(&e);
        transfer_buy(&e, admin, balance_buy_token);
    }

    fn withdraw(e: Env, amount: i128) {
        let invoker = e.invoker().into();
        check_admin(&e, &invoker);
        transfer_sell(&e, invoker, amount);
    }

    fn updt_price(e: Env, n: u32, d: u32) {
        check_admin(&e, &e.invoker().into());

        if d == 0 {
            panic!("d is zero but cannot be zero")
        }
        put_price(&e, Price { n, d });
    }

    fn get_price(e: Env) -> Price {
        load_price(&e)
    }

    fn get_sell(e: Env) -> BytesN<32> {
        get_sell_token(&e)
    }

    fn get_buy(e: Env) -> BytesN<32> {
        get_buy_token(&e)
    }
}
