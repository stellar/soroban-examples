#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod offer_contract;
mod test;
pub mod testutils;
mod token_contract;

use offer_contract::{create_contract, SingleOfferClient};
use token_contract::TokenClient;

use soroban_auth::{
    verify, {Identifier, Signature},
};
use soroban_sdk::{
    contractimpl, contracttype, serde::Serialize, BigInt, Bytes, BytesN, Env, Symbol,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer(BytesN<32>),
    Nonce(Identifier),
}

fn get_offer(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    e.data()
        .get_unchecked(DataKey::Offer(salt.clone()))
        .unwrap()
}

fn put_offer(e: &Env, salt: &BytesN<32>, offer: &BytesN<32>) {
    e.data().set(DataKey::Offer(salt.clone()), offer.clone())
}

fn has_offer(e: &Env, salt: &BytesN<32>) -> bool {
    e.data().has(DataKey::Offer(salt.clone()))
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
    fn safe_trade(
        e: Env,
        to: Signature,
        nonce: BigInt,
        offer: BytesN<32>,
        amount: BigInt,
        min: BigInt,
    );

    // returns the contract address for the specified admin, sell_token, buy_token combo
    fn get_offer(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
    ) -> BytesN<32>;

    // Returns the current nonce for "id"
    fn nonce(e: Env, id: Identifier) -> BigInt;
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
        Identifier::Account(a) => salt_bin.append(&a.serialize(&e)),
    }
    salt_bin.append(&sell_token.clone().into());
    salt_bin.append(&buy_token.clone().into());
    e.compute_hash_sha256(&salt_bin)
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

        SingleOfferClient::new(&e, offer_contract_id).initialize(
            &admin,
            &sell_token,
            &buy_token,
            &n,
            &d,
        );
    }

    fn safe_trade(
        e: Env,
        to: Signature,
        nonce: BigInt,
        offer: BytesN<32>,
        amount: BigInt,
        min: BigInt,
    ) {
        let to_id = to.identifier(&e);

        verify_and_consume_nonce(&e, &to_id, &nonce);

        verify(
            &e,
            &to,
            Symbol::from_str("safe_trade"),
            (&to_id, nonce, &offer, &amount, &min),
        );

        // TODO:specify buy token instead of calling into offer contract?
        let offer_client = SingleOfferClient::new(&e, &offer);
        let buy = offer_client.get_buy();

        let token_client = TokenClient::new(&e, &buy);

        token_client.xfer_from(
            &Signature::Invoker,
            &BigInt::zero(&e),
            &to_id,
            &Identifier::Contract(offer.clone()),
            &amount,
        );

        offer_client.trade(&to_id, &min);
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

    fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, &id)
    }
}
