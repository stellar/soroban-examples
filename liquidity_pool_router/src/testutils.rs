#![cfg(any(test, feature = "testutils"))]
use soroban_sdk::{AccountId, BigInt, BytesN, Env};

use crate::LiquidityPoolRouterClient;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::LiquidityPoolRouter {});
}

pub struct LiquidityPoolRouter {
    env: Env,
    contract_id: BytesN<32>,
}

impl LiquidityPoolRouter {
    fn client(&self) -> LiquidityPoolRouterClient {
        LiquidityPoolRouterClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }

    pub fn sf_deposit(
        &self,
        to: &AccountId,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
        desired_a: &BigInt,
        min_a: &BigInt,
        desired_b: &BigInt,
        min_b: &BigInt,
    ) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        self.client()
            .with_source_account(&to)
            .sf_deposit(&token_a, &token_b, &desired_a, &min_a, &desired_b, &min_b)
    }

    pub fn swap_out(
        &self,
        to: &AccountId,
        sell: &[u8; 32],
        buy: &[u8; 32],
        out: &BigInt,
        in_max: &BigInt,
    ) {
        let sell = BytesN::from_array(&self.env, sell);
        let buy = BytesN::from_array(&self.env, buy);

        self.client()
            .with_source_account(&to)
            .swap_out(&sell, &buy, &out, &in_max)
    }

    pub fn sf_withdrw(
        &self,
        to: &AccountId,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
        share_amount: &BigInt,
        min_a: &BigInt,
        min_b: &BigInt,
    ) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        self.client().with_source_account(&to).sf_withdrw(
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

        self.client().get_pool(&token_a, &token_b)
    }
}
