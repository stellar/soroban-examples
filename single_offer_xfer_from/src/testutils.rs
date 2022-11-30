#![cfg(any(test, feature = "testutils"))]

use crate::{token::Identifier, Price, SingleOfferXferFromClient};
use soroban_sdk::{AccountId, BigInt, BytesN, Env};

pub fn register_test_contract(e: &Env) -> BytesN<32> {
    e.register_contract(None, crate::SingleOfferXferFrom {})
}

pub struct SingleOfferXferFrom {
    env: Env,
    pub contract_id: BytesN<32>,
}

impl SingleOfferXferFrom {
    fn client(&self) -> SingleOfferXferFromClient {
        SingleOfferXferFromClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &BytesN<32>) -> Self {
        Self {
            env: env.clone(),
            contract_id: contract_id.clone(),
        }
    }

    pub fn initialize(
        &self,
        admin: &Identifier,
        token_a: &BytesN<32>,
        token_b: &BytesN<32>,
        n: u32,
        d: u32,
    ) {
        self.client().initialize(admin, token_a, token_b, &n, &d)
    }

    pub fn trade(&self, to: &AccountId, amount_to_sell: &BigInt, min: &BigInt) {
        self.client()
            .with_source_account(&to)
            .trade(&amount_to_sell, &min)
    }

    pub fn updt_price(&self, admin: &AccountId, n: u32, d: u32) {
        self.client().with_source_account(&admin).updt_price(&n, &d)
    }

    pub fn get_price(&self) -> Price {
        self.client().get_price()
    }
}
