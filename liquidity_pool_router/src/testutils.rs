#![cfg(any(test, feature = "testutils"))]
use soroban_sdk::{BigInt, BytesN, Env};
use soroban_sdk_auth::public_types::Identifier;

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

        self.client().sf_deposit(
            to.clone(),
            token_a,
            token_b,
            desired_a.clone(),
            min_a.clone(),
            desired_b.clone(),
            min_b.clone(),
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

        self.client()
            .swap_out(to.clone(), sell, buy, out.clone(), in_max.clone())
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

        self.client().sf_withdrw(
            to.clone(),
            token_a,
            token_b,
            share_amount.clone(),
            min_a.clone(),
            min_b.clone(),
        )
    }

    pub fn get_pool(&self, token_a: &[u8; 32], token_b: &[u8; 32]) -> BytesN<32> {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        self.client().get_pool(token_a, token_b)
    }
}
