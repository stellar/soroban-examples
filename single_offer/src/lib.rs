#![no_std]

#[cfg(feature = "external")]
extern crate std;

pub mod external;
mod test;
use stellar_contract_sdk::{
    contractimpl, contracttype, BigInt, Binary, Env, EnvVal, IntoEnvVal, RawVal, Symbol, Vec,
};
use stellar_token_contract::public_types::{Identifier, KeyedAuthorization, U256};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    SellToken = 0,
    BuyToken = 1,
    Admin = 2,
    Price = 3,
}

impl IntoEnvVal<Env, RawVal> for DataKey {
    fn into_env_val(self, env: &Env) -> EnvVal {
        (self as u32).into_env_val(env)
    }
}

// Price is 1 unit of selling in terms of buying. For example, if you wanted
// to sell 30 XLM and buy 5 BTC, the price would be Price{n: 5, d: 30}.
#[derive(Clone)]
#[contracttype]
struct Price {
    pub n: u32,
    pub d: u32,
}

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract().into())
}

fn get_sell_token(e: &Env) -> Binary {
    e.get_contract_data(DataKey::SellToken)
}

fn get_buy_token(e: &Env) -> Binary {
    e.get_contract_data(DataKey::BuyToken)
}

fn get_balance(e: &Env, contract_id: Binary) -> BigInt {
    let mut args: Vec<EnvVal> = Vec::new(&e);
    args.push(get_contract_id(e).into_env_val(&e));
    e.call(contract_id, Symbol::from_str("balance"), args)
}

fn get_balance_sell(e: &Env) -> BigInt {
    get_balance(&e, get_sell_token(&e))
}

fn get_balance_buy(e: &Env) -> BigInt {
    get_balance(&e, get_buy_token(&e))
}

fn put_sell_token(e: &Env, contract_id: U256) {
    e.put_contract_data(DataKey::SellToken, contract_id);
}

fn put_buy_token(e: &Env, contract_id: U256) {
    e.put_contract_data(DataKey::BuyToken, contract_id);
}

fn put_price(e: &Env, price: Price) {
    e.put_contract_data(DataKey::Price, price);
}

fn get_price(e: &Env) -> Price {
    e.get_contract_data(DataKey::Price)
}

fn transfer(e: &Env, contract_id: Binary, to: Identifier, amount: BigInt) {
    let mut args = Vec::new(e);
    args.push(KeyedAuthorization::Contract.into_env_val(e));
    args.push(to.into_env_val(e));
    args.push(amount.into_env_val(e));
    e.call::<()>(contract_id, Symbol::from_str("xfer"), args);
}

fn transfer_sell(e: &Env, to: Identifier, amount: BigInt) {
    transfer(&e, get_sell_token(&e), to, amount);
}

fn transfer_buy(e: &Env, to: Identifier, amount: BigInt) {
    transfer(&e, get_buy_token(&e), to, amount);
}

//TODO: Expose from token contract?
fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.has_contract_data(key)
}

fn read_administrator(e: &Env) -> Identifier {
    let key = DataKey::Admin;
    e.get_contract_data(key)
}

fn write_administrator(e: &Env, id: Identifier) {
    let key = DataKey::Admin;
    e.put_contract_data(key, id);
}

pub trait SingleOfferTrait {
    // See comment above the Price struct for information on pricing
    fn initialize(e: Env, admin: Identifier, sell_token: U256, buy_token: U256, n: u32, d: u32);
    fn trade(e: Env, to: Identifier, min: u32);
    fn withdraw(e: Env);
}

struct SingleOffer;

#[contractimpl(export_if = "export", tests_if = "external")]
impl SingleOfferTrait for SingleOffer {
    fn initialize(e: Env, admin: Identifier, sell_token: U256, buy_token: U256, n: u32, d: u32) {
        if has_administrator(&e) || n == 0 {
            panic!()
        }
        write_administrator(&e, admin);

        put_sell_token(&e, sell_token);
        put_buy_token(&e, buy_token);
        put_price(&e, Price { n, d });
    }

    fn trade(e: Env, to: Identifier, min: u32) {
        let balance_buy_token = get_balance_buy(&e);

        let price = get_price(&e);

        let amount = balance_buy_token.clone() * BigInt::from_u32(&e, price.d)
            / BigInt::from_u32(&e, price.n);

        if amount < BigInt::from_u32(&e, min) {
            panic!();
        }

        transfer_sell(&e, to, amount);

        let admin = read_administrator(&e);
        transfer_buy(&e, admin, balance_buy_token);
    }

    // TODO: auth check on admin
    fn withdraw(e: Env) {
        let admin = read_administrator(&e);
        transfer_sell(&e, admin, get_balance_sell(&e));
    }

    //TODO: update price
}
