#![cfg(any(test, feature = "testutils"))]
use soroban_sdk::{AccountId, BytesN, Env};

use crate::LiquidityPoolRouterClient;

pub fn register_test_contract(e: &Env) -> BytesN<32> {
    e.register_contract(None, crate::LiquidityPoolRouter {})
}

pub struct LiquidityPoolRouter {
    env: Env,
    pub contract_id: BytesN<32>,
}

impl LiquidityPoolRouter {
    fn client(&self) -> LiquidityPoolRouterClient {
        LiquidityPoolRouterClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &BytesN<32>) -> Self {
        Self {
            env: env.clone(),
            contract_id: contract_id.clone(),
        }
    }

    pub fn sf_deposit(
        &self,
        liquidity_pool_wasm_hash: &BytesN<32>,
        to: &AccountId,
        token_a: &BytesN<32>,
        token_b: &BytesN<32>,
        desired_a: &i128,
        min_a: &i128,
        desired_b: &i128,
        min_b: &i128,
    ) {
        self.client().with_source_account(to).sf_deposit(
            liquidity_pool_wasm_hash,
            token_a,
            token_b,
            desired_a,
            min_a,
            desired_b,
            min_b,
        )
    }

    pub fn swap_out(
        &self,
        to: &AccountId,
        sell: &BytesN<32>,
        buy: &BytesN<32>,
        out: &i128,
        in_max: &i128,
    ) {
        self.client()
            .with_source_account(to)
            .swap_out(sell, buy, out, in_max)
    }

    pub fn sf_withdrw(
        &self,
        to: &AccountId,
        token_a: &BytesN<32>,
        token_b: &BytesN<32>,
        share_amount: &i128,
        min_a: &i128,
        min_b: &i128,
    ) {
        self.client().with_source_account(to).sf_withdrw(
            token_a,
            token_b,
            share_amount,
            min_a,
            min_b,
        )
    }

    pub fn get_pool(&self, token_a: &BytesN<32>, token_b: &BytesN<32>) -> BytesN<32> {
        self.client().get_pool(token_a, token_b)
    }
}
