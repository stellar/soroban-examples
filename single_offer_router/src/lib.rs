#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod offer_contract;
mod test;
pub mod testutils;

use offer_contract::create_contract;
use soroban_sdk::{contractimpl, contracttype, BigInt, Bytes, BytesN, Env};
use soroban_sdk_auth::public_types::{Identifier, Signature};
use soroban_single_offer_contract as offer;
use soroban_token_contract as token;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer(BytesN<32>),
}

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract().into())
}

fn get_offer(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    e.contract_data()
        .get_unchecked(DataKey::Offer(salt.clone()))
        .unwrap()
}

fn put_offer(e: &Env, salt: &BytesN<32>, offer: &BytesN<32>) {
    e.contract_data()
        .set(DataKey::Offer(salt.clone()), offer.clone())
}

fn has_offer(e: &Env, salt: &BytesN<32>) -> bool {
    e.contract_data().has(DataKey::Offer(salt.clone()))
}

pub trait SingleOfferRouterTrait {
    // See comment above the Price struct for information on pricing
    // Creates an offer contract and stores the address in a map
    fn init(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
        n: u32,
        d: u32,
    );

    // TODO: Add auth

    // This contract pulls amount from "to", sends it to "offer", and then calls trade on "offer".
    // The admin must send the sell_token to the offer address specified in this function,
    // and the "to" identifier must set a buy_token allowance for this router contract
    fn safe_trade(e: Env, to: Identifier, offer: BytesN<32>, amount: BigInt, min: BigInt);

    // returns the contract address for the specified admin, sell_token, buy_token combo
    fn get_offer(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
    ) -> BytesN<32>;
}

pub fn offer_salt(
    e: &Env,
    admin: &Identifier,
    sell_token: &BytesN<32>,
    buy_token: &BytesN<32>,
) -> BytesN<32> {
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

#[contractimpl]
impl SingleOfferRouterTrait for SingleOfferRouter {
    fn init(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
        n: u32,
        d: u32,
    ) {
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

    fn safe_trade(e: Env, to: Identifier, offer: BytesN<32>, amount: BigInt, min: BigInt) {
        // TODO:specify buy token instead of calling into offer contract?
        let buy = &offer::get_buy(&e, &offer);
        let nonce = token::nonce(&e, &buy, &get_contract_id(&e));
        token::xfer_from(
            &e,
            &buy,
            &Signature::Contract,
            &nonce,
            &to,
            &Identifier::Contract(offer.clone()),
            &amount,
        );

        offer::trade(&e, &offer, &to, &min);
    }

    fn get_offer(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
    ) -> BytesN<32> {
        let salt = offer_salt(&e, &admin, &sell_token, &buy_token);
        get_offer(&e, &salt)
    }
}
