#![cfg(any(test, feature = "testutils"))]

use crate::{token::Identifier, Price, SingleOfferClient};

use soroban_sdk::{AccountId, BigInt, BytesN, Env};

pub fn register_test_contract(e: &Env) -> BytesN<32> {
    e.register_contract(None, crate::SingleOffer {})
}

pub struct SingleOffer {
    env: Env,
    pub contract_id: BytesN<32>,
}

impl SingleOffer {
    fn client(&self) -> SingleOfferClient {
        SingleOfferClient::new(&self.env, &self.contract_id)
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
        self.client().initialize(&admin, token_a, token_b, &n, &d);
    }

    pub fn trade(&self, to: &Identifier, min: &BigInt) {
        self.client().trade(&to, &min)
    }

    pub fn withdraw(&self, admin: &AccountId, amount: &BigInt) {
        self.client().with_source_account(&admin).withdraw(&amount)
    }

    pub fn updt_price(&self, admin: &AccountId, n: u32, d: u32) {
        self.client().with_source_account(&admin).updt_price(&n, &d)
    }

    pub fn get_price(&self) -> Price {
        self.client().get_price()
    }

    pub fn get_sell(&self) -> BytesN<32> {
        self.client().get_sell()
    }

    pub fn get_buy(&self) -> BytesN<32> {
        self.client().get_buy()
    }
}
