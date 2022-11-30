#![cfg(any(test, feature = "testutils"))]

use soroban_sdk::{BytesN, Env};

use crate::{token::Identifier, LiquidityPoolClient};

pub fn register_test_contract(e: &Env) -> BytesN<32> {
    e.register_contract(None, crate::LiquidityPool {})
}

pub struct LiquidityPool {
    env: Env,
    pub contract_id: BytesN<32>,
}

impl LiquidityPool {
    fn client(&self) -> LiquidityPoolClient {
        LiquidityPoolClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &BytesN<32>) -> Self {
        Self {
            env: env.clone(),
            contract_id: contract_id.clone(),
        }
    }

    pub fn initialize(&self, token_a: &BytesN<32>, token_b: &BytesN<32>) {
        self.client().initialize(token_a, token_b)
    }

    pub fn share_id(&self) -> BytesN<32> {
        self.client().share_id()
    }

    pub fn deposit(&self, to: &Identifier) {
        self.client().deposit(to)
    }

    pub fn swap(&self, to: &Identifier, out_a: &i128, out_b: &i128) {
        self.client().swap(to, out_a, out_b)
    }

    pub fn withdraw(&self, to: &Identifier) -> (i128, i128) {
        self.client().withdraw(to)
    }

    pub fn get_rsrvs(&self) -> (i128, i128) {
        self.client().get_rsrvs()
    }
}
