#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod cryptography;
mod test;
pub mod testutils;

use soroban_sdk::{contractimpl, contracttype, vec, BigInt, Env, FixedBinary, IntoVal, RawVal};
use soroban_token_contract as token;
use token::public_types::{
    Authorization, Identifier, KeyedAccountAuthorization, KeyedAuthorization,
    KeyedEd25519Authorization, U256,
};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    SellToken = 0,
    BuyToken = 1,
    Admin = 2,
    Price = 3,
    Nonce = 4,
}

impl IntoVal<Env, RawVal> for DataKey {
    fn into_val(self, env: &Env) -> RawVal {
        (self as u32).into_val(env)
    }
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
    Identifier::Contract(e.get_current_contract().into())
}

fn get_sell_token(e: &Env) -> FixedBinary<32> {
    e.contract_data().get_unchecked(DataKey::SellToken).unwrap()
}

fn get_buy_token(e: &Env) -> FixedBinary<32> {
    e.contract_data().get_unchecked(DataKey::BuyToken).unwrap()
}

fn get_balance(e: &Env, contract_id: FixedBinary<32>) -> BigInt {
    token::balance(e, &contract_id, &get_contract_id(e))
}

fn get_balance_buy(e: &Env) -> BigInt {
    get_balance(&e, get_buy_token(&e))
}

fn put_sell_token(e: &Env, contract_id: U256) {
    e.contract_data().set(DataKey::SellToken, contract_id);
}

fn put_buy_token(e: &Env, contract_id: U256) {
    e.contract_data().set(DataKey::BuyToken, contract_id);
}

fn put_price(e: &Env, price: Price) {
    e.contract_data().set(DataKey::Price, price);
}

fn get_price(e: &Env) -> Price {
    e.contract_data().get_unchecked(DataKey::Price).unwrap()
}

fn transfer(e: &Env, contract_id: FixedBinary<32>, to: Identifier, amount: BigInt) {
    token::xfer(e, &contract_id, &KeyedAuthorization::Contract, &to, &amount);
}

fn transfer_sell(e: &Env, to: Identifier, amount: BigInt) {
    transfer(&e, get_sell_token(&e), to, amount);
}

fn transfer_buy(e: &Env, to: Identifier, amount: BigInt) {
    transfer(&e, get_buy_token(&e), to, amount);
}

fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.contract_data().has(key)
}

fn read_administrator(e: &Env) -> Identifier {
    let key = DataKey::Admin;
    e.contract_data().get_unchecked(key).unwrap()
}

fn write_administrator(e: &Env, id: Identifier) {
    let key = DataKey::Admin;
    e.contract_data().set(key, id);
}

pub fn to_administrator_authorization(e: &Env, auth: Authorization) -> KeyedAuthorization {
    let admin = read_administrator(e);
    match (admin, auth) {
        (Identifier::Contract(admin_id), Authorization::Contract) => {
            if admin_id != e.get_invoking_contract() {
                panic!("admin is not invoking contract");
            }
            KeyedAuthorization::Contract
        }
        (Identifier::Ed25519(admin_id), Authorization::Ed25519(signature)) => {
            KeyedAuthorization::Ed25519(KeyedEd25519Authorization {
                public_key: admin_id,
                signature,
            })
        }
        (Identifier::Account(admin_id), Authorization::Account(aa)) => {
            KeyedAuthorization::Account(KeyedAccountAuthorization {
                public_key: admin_id,
                auth: aa,
            })
        }
        _ => panic!("unknown identifier type"),
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
   The sellers nonce is required to create the Authorization, which can be retrieved by calling nonce()
*/
pub trait SingleOfferTrait {
    // See comment above the Price struct for information on pricing
    // Sets the admin, the sell/buy tokens, and the price
    fn initialize(e: Env, admin: Identifier, sell_token: U256, buy_token: U256, n: u32, d: u32);

    // Returns the nonce for the admin
    fn nonce(e: Env) -> BigInt;

    // Sends the full balance of this contracts buy_token balance (let's call this BuyB) to the admin, and
    // also sends buyB * d / n of the sell_token to the "to" identifier specified in trade call. Note that
    // the seller and the buyer need to transfer the sell_token and buy_token to this contract prior to calling
    // trade. Due to this and the fact that the buyer is a parameter to trade, the buyer must tranfer the buy_token
    // to the contract and call trade in the same transaction for safety.
    fn trade(e: Env, to: Identifier, min: BigInt);

    // Sends amount of sell_token from this contract to the admin. Must be authorized by admin
    fn withdraw(e: Env, admin: Authorization, amount: BigInt);

    // Updates the price. Must be authorized by admin
    fn updt_price(e: Env, admin: Authorization, n: u32, d: u32);

    // Get the current price
    fn get_price(e: Env) -> Price;
}

struct SingleOffer;

#[contractimpl(export_if = "export")]
impl SingleOfferTrait for SingleOffer {
    fn initialize(e: Env, admin: Identifier, sell_token: U256, buy_token: U256, n: u32, d: u32) {
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

    fn trade(e: Env, to: Identifier, min: BigInt) {
        let balance_buy_token = get_balance_buy(&e);

        let price = get_price(&e);

        let amount = balance_buy_token.clone() * BigInt::from_u32(&e, price.d)
            / BigInt::from_u32(&e, price.n);

        if amount < min {
            panic!("will receive less than min");
        }

        transfer_sell(&e, to, amount);

        let admin = read_administrator(&e);
        transfer_buy(&e, admin, balance_buy_token);
    }

    fn nonce(e: Env) -> BigInt {
        cryptography::read_nonce(&e)
    }

    fn withdraw(e: Env, admin: Authorization, amount: BigInt) {
        let auth = to_administrator_authorization(&e, admin.clone());
        cryptography::check_auth(
            &e,
            auth,
            cryptography::Domain::Withdraw,
            (vec![&e, amount.clone()]).into_env_val(&e),
        );

        transfer_sell(&e, read_administrator(&e), amount);
    }

    fn updt_price(e: Env, admin: Authorization, n: u32, d: u32) {
        if d == 0 {
            panic!("d is zero but cannot be zero")
        }
        let auth = to_administrator_authorization(&e, admin.clone());
        cryptography::check_auth(
            &e,
            auth,
            cryptography::Domain::UpdatePrice,
            (n.clone(), d.clone()).into_env_val(&e),
        );

        put_price(&e, Price { n, d });
    }

    fn get_price(e: Env) -> Price {
        get_price(&e)
    }
}
