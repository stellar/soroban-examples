#![cfg(any(test, feature = "testutils"))]

use crate::cryptography::Domain;
use crate::Price;
use ed25519_dalek::Keypair;
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{BigInt, BytesN, Env, EnvVal, IntoVal, Vec};
use soroban_token_contract::public_types::{Authorization, Identifier, Message, MessageV0};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, *contract_id);
    e.register_contract(&contract_id, crate::SingleOffer {});
}

pub struct SingleOffer {
    env: Env,
    contract_id: BytesN<32>,
}

impl SingleOffer {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, *contract_id),
        }
    }

    pub fn initialize(
        &self,
        admin: &Identifier,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
        n: u32,
        d: u32,
    ) {
        let token_a = BytesN::from_array(&self.env, *token_a);
        let token_b = BytesN::from_array(&self.env, *token_b);
        crate::initialize(
            &self.env,
            &self.contract_id,
            admin,
            &token_a,
            &token_b,
            &n,
            &d,
        )
    }

    pub fn nonce(&self) -> BigInt {
        crate::nonce(&self.env, &self.contract_id)
    }

    pub fn trade(&self, to: &Identifier, min: &BigInt) {
        crate::trade(&self.env, &self.contract_id, &to, &min)
    }

    pub fn withdraw(&self, admin: &Keypair, amount: &BigInt) {
        let mut args: Vec<EnvVal> = Vec::new(&self.env);
        args.push(amount.clone().into_env_val(&self.env));
        let msg = Message::V0(MessageV0 {
            nonce: self.nonce(),
            domain: Domain::Withdraw as u32,
            parameters: args,
        });
        let auth = Authorization::Ed25519(admin.sign(msg).unwrap().into_val(&self.env));
        crate::withdraw(&self.env, &self.contract_id, &auth, amount)
    }

    pub fn updt_price(&self, admin: &Keypair, n: u32, d: u32) {
        let mut args: Vec<EnvVal> = Vec::new(&self.env);
        args.push(n.into_env_val(&self.env));
        args.push(d.into_env_val(&self.env));
        let msg = Message::V0(MessageV0 {
            nonce: self.nonce(),
            domain: Domain::UpdatePrice as u32,
            parameters: args,
        });
        let auth = Authorization::Ed25519(admin.sign(msg).unwrap().into_val(&self.env));
        crate::updt_price(&self.env, &self.contract_id, &auth, &n, &d)
    }

    pub fn get_price(&self) -> Price {
        crate::get_price(&self.env, &self.contract_id)
    }

    pub fn get_sell(&self) -> BytesN<32> {
        crate::get_sell(&self.env, &self.contract_id)
    }

    pub fn get_buy(&self) -> BytesN<32> {
        crate::get_buy(&self.env, &self.contract_id)
    }
}
