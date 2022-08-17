#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod offer_contract;
mod test;
pub mod testutils;

use offer_contract::create_contract;
use soroban_sdk::{contractimpl, contracttype, BigInt, Bytes, BytesN, Env};
use soroban_single_offer_contract as offer;
use soroban_token_contract as token;
use token::public_types::{Identifier, KeyedAuthorization, U256};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer(U256),
}

fn get_offer(e: &Env, salt: &U256) -> U256 {
    e.contract_data()
        .get_unchecked(DataKey::Offer(salt.clone()))
        .unwrap()
}

fn put_offer(e: &Env, salt: &U256, offer: &U256) {
    e.contract_data()
        .set(DataKey::Offer(salt.clone()), offer.clone())
}

fn has_offer(e: &Env, salt: &U256) -> bool {
    e.contract_data().has(DataKey::Offer(salt.clone()))
}

pub trait SingleOfferRouterTrait {
    // See comment above the Price struct for information on pricing
    // Creates an offer contract and stores the address in a map
    fn init(e: Env, admin: Identifier, sell_token: U256, buy_token: U256, n: u32, d: u32);

    // TODO: Add auth

    // This contract pulls amount from "to", sends it to "offer", and then calls trade on "offer".
    // The admin must send the sell_token to the offer address specified in this function,
    // and the "to" identifier must set a buy_token allowance for this router contract
    fn safe_trade(e: Env, to: Identifier, offer: U256, amount: BigInt, min: BigInt);

    // returns the contract address for the specified admin, sell_token, buy_token combo
    fn get_offer(e: Env, admin: Identifier, sell_token: U256, buy_token: U256) -> U256;
}

pub fn offer_salt(e: &Env, admin: &Identifier, sell_token: &U256, buy_token: &U256) -> U256 {
    let mut salt_bin = Bytes::new(&e);

    match admin {
        Identifier::Contract(a) => salt_bin.append(&a.clone().into()),
        Identifier::Ed25519(a) => salt_bin.append(&a.clone().into()),
        Identifier::Account(a) => salt_bin.append(&a.clone().into()),
    }
    salt_bin.append(&sell_token.clone().into());
    salt_bin.append(&buy_token.clone().into());
    e.compute_hash_sha256(salt_bin)
}

struct SingleOfferRouter;

#[contractimpl(export_if = "export")]
impl SingleOfferRouterTrait for SingleOfferRouter {
    fn init(e: Env, admin: Identifier, sell_token: U256, buy_token: U256, n: u32, d: u32) {
        let salt = offer_salt(&e, &admin, &sell_token, &buy_token);

        if has_offer(&e, &salt) {
            panic!("contract already exists");
        }

        let offer_contract_id = create_contract(&e, &salt);

        put_offer(&e, &salt, &offer_contract_id);

        offer::initialize(
            &e,
            &offer_contract_id,
            &admin,
            &sell_token,
            &buy_token,
            &n,
            &d,
        );
    }

    fn safe_trade(e: Env, to: Identifier, offer: U256, amount: BigInt, min: BigInt) {
        // TODO:specify buy token instead of calling into offer contract?
        token::xfer_from(
            &e,
            &offer::get_buy(&e, &offer),
            &KeyedAuthorization::Contract,
            &to,
            &Identifier::Contract(offer.clone()),
            &amount,
        );

        offer::trade(&e, &offer, &to, &min);
    }

    fn get_offer(e: Env, admin: Identifier, sell_token: U256, buy_token: U256) -> BytesN<32> {
        let salt = offer_salt(&e, &admin, &sell_token, &buy_token);
        get_offer(&e, &salt)
    }
}
