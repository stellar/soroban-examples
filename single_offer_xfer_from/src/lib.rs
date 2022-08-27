#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod test;
pub mod testutils;

use soroban_sdk::{contractimpl, contracttype, vec, BigInt, BytesN, Env, IntoVal, Symbol};
use soroban_sdk_auth::{
    check_auth,
    public_types::{Identifier, Signature},
    NonceAuth,
};
use soroban_token_contract as token;
use token::TokenClient;

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

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract().into())
}

fn get_sell_token(e: &Env) -> BytesN<32> {
    e.contract_data().get_unchecked(DataKey::SellToken).unwrap()
}

fn get_buy_token(e: &Env) -> BytesN<32> {
    e.contract_data().get_unchecked(DataKey::BuyToken).unwrap()
}

fn put_sell_token(e: &Env, contract_id: BytesN<32>) {
    e.contract_data().set(DataKey::SellToken, contract_id);
}

fn put_buy_token(e: &Env, contract_id: BytesN<32>) {
    e.contract_data().set(DataKey::BuyToken, contract_id);
}

fn put_price(e: &Env, price: Price) {
    e.contract_data().set(DataKey::Price, price);
}

fn get_price(e: &Env) -> Price {
    e.contract_data().get_unchecked(DataKey::Price).unwrap()
}

fn transfer_from(
    e: &Env,
    contract_id: BytesN<32>,
    from: &Identifier,
    to: &Identifier,
    amount: &BigInt,
) {
    let client = TokenClient::new(&e, &contract_id);
    let nonce = client.nonce(get_contract_id(&e));
    client.xfer_from(
        Signature::Contract,
        nonce,
        from.clone(),
        to.clone(),
        amount.clone(),
    )
}

fn transfer_sell(e: &Env, from: &Identifier, to: &Identifier, amount: &BigInt) {
    transfer_from(&e, get_sell_token(&e), from, to, amount);
}

fn transfer_buy(e: &Env, from: &Identifier, to: &Identifier, amount: &BigInt) {
    transfer_from(&e, get_buy_token(&e), from, to, amount);
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

pub fn check_admin(e: &Env, auth: &Signature) {
    let auth_id = auth.get_identifier(&e);
    if auth_id != read_administrator(&e) {
        panic!("not authorized by admin")
    }
}

fn read_nonce(e: &Env, id: Identifier) -> BigInt {
    let key = DataKey::Nonce(id);
    if let Some(nonce) = e.contract_data().get(key) {
        nonce.unwrap()
    } else {
        BigInt::zero(e)
    }
}
struct WrappedAuth(Signature);

impl NonceAuth for WrappedAuth {
    fn read_nonce(e: &Env, id: Identifier) -> BigInt {
        read_nonce(e, id)
    }

    fn read_and_increment_nonce(&self, e: &Env, id: Identifier) -> BigInt {
        let key = DataKey::Nonce(id.clone());
        let nonce = Self::read_nonce(e, id);
        e.contract_data()
            .set(key, nonce.clone() + BigInt::from_u32(e, 1));
        nonce
    }

    fn get_keyed_auth(&self) -> &Signature {
        &self.0
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
        let to_id = to.get_identifier(&e);
        check_auth(
            &e,
            &WrappedAuth(to),
            nonce.clone(),
            Symbol::from_str("trade"),
            vec![
                &e,
                nonce.into_val(&e),
                amount_to_sell.clone().into_val(&e),
                min.clone().into_val(&e),
            ],
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
        read_nonce(&e, id)
    }

    fn updt_price(e: Env, admin: Signature, nonce: BigInt, n: u32, d: u32) {
        check_admin(&e, &admin);

        if d == 0 {
            panic!("d is zero but cannot be zero")
        }

        check_auth(
            &e,
            &WrappedAuth(admin),
            nonce.clone(),
            Symbol::from_str("updt_price"),
            vec![
                &e,
                nonce.into_val(&e),
                n.clone().into_val(&e),
                d.clone().into_val(&e),
            ],
        );

        put_price(&e, Price { n, d });
    }

    fn get_price(e: Env) -> Price {
        get_price(&e)
    }
}
