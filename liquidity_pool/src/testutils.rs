#![cfg(any(test, feature = "testutils"))]

use soroban_sdk::{BigInt, BytesN, Env};
use soroban_sdk_auth::Identifier;

use crate::LiquidityPoolClient;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::LiquidityPool {});
}

pub struct LiquidityPool {
    env: Env,
    contract_id: BytesN<32>,
}

impl LiquidityPool {
    fn client(&self) -> LiquidityPoolClient {
        LiquidityPoolClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }

    pub fn initialize(&self, token_a: &[u8; 32], token_b: &[u8; 32]) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);
        self.client().initialize(&token_a, &token_b)
    }

    pub fn share_id(&self) -> BytesN<32> {
        self.client().share_id()
    }

    pub fn deposit(&self, to: &Identifier) {
        self.client().deposit(&to)
    }

    pub fn swap(&self, to: &Identifier, out_a: &BigInt, out_b: &BigInt) {
        self.client().swap(&to, &out_a, &out_b)
    }

    pub fn withdraw(&self, to: &Identifier) -> (BigInt, BigInt) {
        self.client().withdraw(&to)
    }

    pub fn get_rsrvs(&self) -> (BigInt, BigInt) {
        self.client().get_rsrvs()
    }
}
