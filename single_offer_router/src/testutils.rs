#![cfg(any(test, feature = "testutils"))]
use soroban_sdk::{AccountId, BigInt, BytesN, Env};

use crate::{token::Identifier, SingleOfferRouterClient};

pub fn register_test_contract(e: &Env) -> BytesN<32> {
    e.register_contract(None, crate::SingleOfferRouter {})
}

pub struct SingleOfferRouter {
    env: Env,
    pub contract_id: BytesN<32>,
}

impl SingleOfferRouter {
    fn client(&self) -> SingleOfferRouterClient {
        SingleOfferRouterClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &BytesN<32>) -> Self {
        Self {
            env: env.clone(),
            contract_id: contract_id.clone(),
        }
    }

    pub fn init(
        &self,
        offer_wasm_hash: &BytesN<32>,
        admin: &Identifier,
        token_a: &BytesN<32>,
        token_b: &BytesN<32>,
        n: u32,
        d: u32,
    ) {
        self.client()
            .init(offer_wasm_hash, admin, token_a, token_b, &n, &d)
    }

    pub fn safe_trade(&self, to: &AccountId, offer: &BytesN<32>, amount: &BigInt, min: &BigInt) {
        self.client()
            .with_source_account(&to)
            .safe_trade(offer, &amount, &min)
    }

    pub fn get_offer(
        &self,
        admin: &Identifier,
        token_a: &BytesN<32>,
        token_b: &BytesN<32>,
    ) -> BytesN<32> {
        self.client().get_offer(admin, token_a, token_b)
    }
}
