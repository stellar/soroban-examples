#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod offer_contract;
mod test;
pub mod testutils;

use offer_contract::{create_contract, SingleOfferClient};

use soroban_auth::{
    verify, {Identifier, Signature},
};
use soroban_sdk::{
    contractimpl, contracttype, serde::Serialize, BigInt, Bytes, BytesN, Env, Symbol,
};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer(BytesN<32>),
    Nonce(Identifier),
}

fn get_offer(e: &Env, offer_key: &BytesN<32>) -> BytesN<32> {
    e.data()
        .get_unchecked(DataKey::Offer(offer_key.clone()))
        .unwrap()
}

fn put_offer(e: &Env, offer_key: &BytesN<32>, offer: &BytesN<32>) {
    e.data()
        .set(DataKey::Offer(offer_key.clone()), offer.clone())
}

fn has_offer(e: &Env, offer_key: &BytesN<32>) -> bool {
    e.data().has(DataKey::Offer(offer_key.clone()))
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

pub fn offer_key(
    e: &Env,
    admin: &Identifier,
    sell_token: &BytesN<32>,
    buy_token: &BytesN<32>,
) -> BytesN<32> {
    let mut offer_key_bin = Bytes::new(&e);

    match admin {
        Identifier::Contract(a) => offer_key_bin.append(&a.clone().into()),
        Identifier::Ed25519(a) => offer_key_bin.append(&a.clone().into()),
        Identifier::Account(a) => offer_key_bin.append(&a.serialize(&e)),
    }
    offer_key_bin.append(&sell_token.clone().into());
    offer_key_bin.append(&buy_token.clone().into());
    e.compute_hash_sha256(&offer_key_bin)
}

fn read_nonce(e: &Env, id: &Identifier) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    e.data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(e)))
        .unwrap()
}

fn verify_and_consume_nonce(e: &Env, auth: &Signature, expected_nonce: &BigInt) {
    match auth {
        Signature::Invoker => {
            if BigInt::zero(&e) != expected_nonce {
                panic!("nonce should be zero for Invoker")
            }
            return;
        }
        _ => {}
    }

    let id = auth.identifier(&e);
    let key = DataKey::Nonce(id.clone());
    let nonce = read_nonce(e, &id);

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
        let offer_key = offer_key(&e, &admin, &sell_token, &buy_token);

        if has_offer(&e, &offer_key) {
            panic!("contract already exists");
        }

        let offer_contract_id = create_contract(&e, &offer_key);

        put_offer(&e, &offer_key, &offer_contract_id);

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
        verify_and_consume_nonce(&e, &to, &nonce);

        let to_id = to.identifier(&e);

        verify(
            &e,
            &to,
            Symbol::from_str("safe_trade"),
            (&to_id, nonce, &offer, &amount, &min),
        );

        // TODO:specify buy token instead of calling into offer contract?
        let offer_client = SingleOfferClient::new(&e, &offer);
        let buy = offer_client.get_buy();

        let token_client = token::Client::new(&e, &buy);

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
        let offer_key = offer_key(&e, &admin, &sell_token, &buy_token);
        get_offer(&e, &offer_key)
    }

    fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, &id)
    }
}
