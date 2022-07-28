#![cfg(feature = "testutils")]

use crate::cryptography::Domain;
use crate::Price;
use ed25519_dalek::Keypair;
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{BigInt, Env, EnvVal, FixedBinary, IntoVal, Vec};
use soroban_token_contract::public_types::{Authorization, Identifier, Message, MessageV0};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = FixedBinary::from_array(e, *contract_id);
    e.register_contract(&contract_id, crate::SingleOffer {});
}

pub use crate::get_price::invoke as get_price;
pub use crate::initialize::invoke as initialize;
pub use crate::nonce::invoke as nonce;
pub use crate::trade::invoke as trade;
pub use crate::updt_price::invoke as updt_price;
pub use crate::withdraw::invoke as withdraw;

pub struct SingleOffer {
    env: Env,
    contract_id: FixedBinary<32>,
}

impl SingleOffer {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: FixedBinary::from_array(env, *contract_id),
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
        let token_a = FixedBinary::from_array(&self.env, *token_a);
        let token_b = FixedBinary::from_array(&self.env, *token_b);
        initialize(
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
        nonce(&self.env, &self.contract_id)
    }

    pub fn trade(&self, to: &Identifier, min: &BigInt) {
        trade(&self.env, &self.contract_id, &to, &min)
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
        withdraw(&self.env, &self.contract_id, &auth, amount)
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
        updt_price(&self.env, &self.contract_id, &auth, &n, &d)
    }

    pub fn get_price(&self) -> Price {
        get_price(&self.env, &self.contract_id)
    }
}
