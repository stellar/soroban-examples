#![cfg(any(test, feature = "testutils"))]
use soroban_sdk::{BigInt, BytesN, Env};
use soroban_sdk_auth::public_types::Identifier;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::LiquidityPoolRouter {});
}

pub struct LiquidityPoolRouter {
    env: Env,
    contract_id: BytesN<32>,
}

impl LiquidityPoolRouter {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }

    pub fn sf_deposit(
        &self,
        to: &Identifier,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
        desired_a: &BigInt,
        min_a: &BigInt,
        desired_b: &BigInt,
        min_b: &BigInt,
    ) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        crate::sf_deposit(
            &self.env,
            &self.contract_id,
            &to,
            &token_a,
            &token_b,
            &desired_a,
            &min_a,
            &desired_b,
            &min_b,
        )
    }

    pub fn swap_out(
        &self,
        to: &Identifier,
        sell: &[u8; 32],
        buy: &[u8; 32],
        out: &BigInt,
        in_max: &BigInt,
    ) {
        let sell = BytesN::from_array(&self.env, sell);
        let buy = BytesN::from_array(&self.env, buy);

        crate::swap_out(
            &self.env,
            &self.contract_id,
            &to,
            &sell,
            &buy,
            &out,
            &in_max,
        )
    }

    pub fn sf_withdrw(
        &self,
        to: &Identifier,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
        share_amount: &BigInt,
        min_a: &BigInt,
        min_b: &BigInt,
    ) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        crate::sf_withdrw(
            &self.env,
            &self.contract_id,
            &to,
            &token_a,
            &token_b,
            &share_amount,
            &min_a,
            &min_b,
        )
    }

    pub fn get_pool(&self, token_a: &[u8; 32], token_b: &[u8; 32]) -> BytesN<32> {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        crate::get_pool(&self.env, &self.contract_id, &token_a, &token_b)
    }
}
