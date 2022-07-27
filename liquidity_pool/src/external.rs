#![cfg(feature = "testutils")]

use stellar_contract_sdk::{BigInt, Binary, Env, FixedBinary};
use stellar_token_contract::public_types::Identifier;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = Binary::from_array(e, *contract_id);
    e.register_contract(contract_id, crate::LiquidityPool {});
}

pub use crate::__deposit::call_external as deposit;
pub use crate::__initialize::call_external as initialize;
pub use crate::__share_id::call_external as share_id;
pub use crate::__swap::call_external as swap;
pub use crate::__withdraw::call_external as withdraw;

pub struct LiquidityPool {
    env: Env,
    contract_id: Binary,
}

impl LiquidityPool {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: Binary::from_slice(env, contract_id),
        }
    }

    pub fn initialize(&mut self, token_a: &[u8; 32], token_b: &[u8; 32]) {
        let token_a = FixedBinary::from_array(&self.env, *token_a);
        let token_b = FixedBinary::from_array(&self.env, *token_b);
        initialize(&mut self.env, &self.contract_id, &token_a, &token_b)
    }

    pub fn share_id(&mut self) -> Binary {
        share_id(&mut self.env, &self.contract_id)
    }

    pub fn deposit(&mut self, to: &Identifier) {
        deposit(&mut self.env, &self.contract_id, to)
    }

    pub fn swap(&mut self, to: &Identifier, out_a: &BigInt, out_b: &BigInt) {
        swap(&mut self.env, &self.contract_id, to, out_a, out_b)
    }

    pub fn withdraw(&mut self, to: &Identifier) {
        withdraw(&mut self.env, &self.contract_id, to)
    }
}
