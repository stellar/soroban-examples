#![no_std]

mod offer_contract;
mod test;
pub mod testutils;

use offer_contract::SingleOfferClient;
use token::{Identifier, Signature};

use soroban_sdk::{contractimpl, contracttype, serde::Serialize, Bytes, BytesN, Env};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer(BytesN<32>),
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
        offer_wasm_hash: BytesN<32>,
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
    fn safe_trade(e: Env, offer: BytesN<32>, amount: i128, min: i128);

    // returns the contract address for the specified admin, sell_token, buy_token combo
    fn get_offer(
        e: Env,
        admin: Identifier,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
    ) -> BytesN<32>;
}

pub fn offer_key(
    e: &Env,
    admin: &Identifier,
    sell_token: &BytesN<32>,
    buy_token: &BytesN<32>,
) -> BytesN<32> {
    let mut offer_key_bin = Bytes::new(e);

    match admin {
        Identifier::Contract(a) => offer_key_bin.append(&a.clone().into()),
        Identifier::Ed25519(a) => offer_key_bin.append(&a.clone().into()),
        Identifier::Account(a) => offer_key_bin.append(&a.serialize(e)),
    }
    offer_key_bin.append(&sell_token.clone().into());
    offer_key_bin.append(&buy_token.clone().into());
    e.crypto().sha256(&offer_key_bin)
}

struct SingleOfferRouter;

#[contractimpl]
impl SingleOfferRouterTrait for SingleOfferRouter {
    fn init(
        e: Env,
        offer_wasm_hash: BytesN<32>,
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

        let offer_contract_id = e
            .deployer()
            .with_current_contract(offer_key.clone())
            .deploy(offer_wasm_hash);

        put_offer(&e, &offer_key, &offer_contract_id);

        SingleOfferClient::new(&e, offer_contract_id).initialize(
            &admin,
            &sell_token,
            &buy_token,
            &n,
            &d,
        );
    }

    fn safe_trade(e: Env, offer: BytesN<32>, amount: i128, min: i128) {
        // TODO:specify buy token instead of calling into offer contract?
        let offer_client = SingleOfferClient::new(&e, &offer);
        let buy = offer_client.get_buy();

        let token_client = token::Client::new(&e, &buy);

        let invoker = e.invoker().into();
        token_client.xfer_from(
            &Signature::Invoker,
            &0,
            &invoker,
            &Identifier::Contract(offer),
            &amount,
        );

        offer_client.trade(&invoker, &min);
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
}
