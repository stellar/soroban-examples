#![cfg(any(test, feature = "testutils"))]
use soroban_sdk::{AccountId, BigInt, BytesN, Env};

use crate::{token::Identifier, SingleOfferRouterClient};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::SingleOfferRouter {});
}

pub struct SingleOfferRouter {
    env: Env,
    contract_id: BytesN<32>,
}

impl SingleOfferRouter {
    fn client(&self) -> SingleOfferRouterClient {
        SingleOfferRouterClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }

    pub fn init(&self, admin: &Identifier, token_a: &[u8; 32], token_b: &[u8; 32], n: u32, d: u32) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);
        self.client().init(&admin, &token_a, &token_b, &n, &d)
    }

    pub fn safe_trade(&self, to: &AccountId, offer: &[u8; 32], amount: &BigInt, min: &BigInt) {
        let offer_addr = BytesN::from_array(&self.env, offer);
        self.client()
            .with_source_account(&to)
            .safe_trade(&offer_addr, &amount, &min)
    }

    pub fn get_offer(
        &self,
        admin: &Identifier,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
    ) -> BytesN<32> {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);
        self.client().get_offer(&admin, &token_a, &token_b)
    }
}
