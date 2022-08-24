#![cfg(any(test, feature = "testutils"))]

use soroban_sdk::{BigInt, BytesN, Env};
use soroban_sdk_auth::public_types::Identifier;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::LiquidityPool {});
}

pub struct LiquidityPool {
    env: Env,
    contract_id: BytesN<32>,
}

impl LiquidityPool {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }

    pub fn initialize(&self, token_a: &[u8; 32], token_b: &[u8; 32]) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);
        crate::initialize(&self.env, &self.contract_id, &token_a, &token_b)
    }

    pub fn share_id(&self) -> BytesN<32> {
        crate::share_id(&self.env, &self.contract_id)
    }

    pub fn deposit(&self, to: &Identifier) {
        crate::deposit(&self.env, &self.contract_id, to)
    }

    pub fn swap(&self, to: &Identifier, out_a: &BigInt, out_b: &BigInt) {
        crate::swap(&self.env, &self.contract_id, to, out_a, out_b)
    }

    pub fn withdraw(&self, to: &Identifier) -> (BigInt, BigInt) {
        crate::withdraw(&self.env, &self.contract_id, to)
    }

    pub fn get_rsrvs(&self) -> (BigInt, BigInt) {
        crate::get_rsrvs(&self.env, &self.contract_id)
    }
}
