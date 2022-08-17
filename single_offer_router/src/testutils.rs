#![cfg(feature = "testutils")]
use soroban_sdk::{BigInt, BytesN, Env};
use soroban_token_contract::public_types::Identifier;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, *contract_id);
    e.register_contract(&contract_id, crate::SingleOfferRouter {});
}

pub use crate::get_offer::invoke as get_offer;
pub use crate::init::invoke as init;
pub use crate::safe_trade::invoke as safe_trade;

pub struct SingleOfferRouter {
    env: Env,
    contract_id: BytesN<32>,
}

impl SingleOfferRouter {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, *contract_id),
        }
    }

    pub fn init(&self, admin: &Identifier, token_a: &[u8; 32], token_b: &[u8; 32], n: u32, d: u32) {
        let token_a = BytesN::from_array(&self.env, *token_a);
        let token_b = BytesN::from_array(&self.env, *token_b);
        init(
            &self.env,
            &self.contract_id,
            admin,
            &token_a,
            &token_b,
            &n,
            &d,
        )
    }

    pub fn safe_trade(&self, to: &Identifier, offer: &[u8; 32], amount: &BigInt, min: &BigInt) {
        let offer_addr = BytesN::from_array(&self.env, *offer);
        safe_trade(
            &self.env,
            &self.contract_id,
            &to,
            &offer_addr,
            &amount,
            &min,
        )
    }

    pub fn get_offer(
        &self,
        admin: &Identifier,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
    ) -> BytesN<32> {
        let token_a = BytesN::from_array(&self.env, *token_a);
        let token_b = BytesN::from_array(&self.env, *token_b);
        get_offer(&self.env, &self.contract_id, &admin, &token_a, &token_b)
    }
}
