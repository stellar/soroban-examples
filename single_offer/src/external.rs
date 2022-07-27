#![cfg(feature = "testutils")]

use crate::cryptography::Domain;
use crate::Price;
use ed25519_dalek::Keypair;
use stellar_contract_sdk::testutils::ed25519::Sign;
use stellar_contract_sdk::{BigInt, Binary, Env, EnvVal, FixedBinary, IntoEnvVal, TryIntoVal, Vec};
use stellar_token_contract::public_types::{Authorization, Identifier, Message, MessageV0};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = Binary::from_array(e, *contract_id);
    e.register_contract(contract_id, crate::SingleOffer {});
}

pub use crate::__get_price::call_external as get_price;
pub use crate::__initialize::call_external as initialize;
pub use crate::__nonce::call_external as nonce;
pub use crate::__trade::call_external as trade;
pub use crate::__updt_price::call_external as updt_price;
pub use crate::__withdraw::call_external as withdraw;

pub struct SingleOffer {
    env: Env,
    contract_id: Binary,
}

impl SingleOffer {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: Binary::from_slice(env, contract_id),
        }
    }

    pub fn initialize(
        &mut self,
        admin: &Identifier,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
        n: u32,
        d: u32,
    ) {
        let token_a = FixedBinary::from_array(&self.env, *token_a);
        let token_b = FixedBinary::from_array(&self.env, *token_b);
        initialize(
            &mut self.env,
            &self.contract_id,
            admin,
            &token_a,
            &token_b,
            &n,
            &d,
        )
    }

    pub fn nonce(&mut self) -> BigInt {
        nonce(&mut self.env, &self.contract_id)
    }

    pub fn trade(&mut self, to: &Identifier, min: u32) {
        trade(&mut self.env, &self.contract_id, &to, &min)
    }

    pub fn withdraw(&mut self, admin: &Keypair, amount: &BigInt) {
        let mut args: Vec<EnvVal> = Vec::new(&self.env);
        args.push(amount.clone().into_env_val(&self.env));
        let msg = Message::V0(MessageV0 {
            nonce: self.nonce(),
            domain: Domain::Withdraw as u32,
            parameters: args,
        });
        let auth =
            Authorization::Ed25519(admin.sign(msg).unwrap().try_into_val(&self.env).unwrap());
        withdraw(&mut self.env, &self.contract_id, &auth, amount)
    }

    pub fn updt_price(&mut self, admin: &Keypair, n: u32, d: u32) {
        let mut args: Vec<EnvVal> = Vec::new(&self.env);
        args.push(n.into_env_val(&self.env));
        args.push(d.into_env_val(&self.env));
        let msg = Message::V0(MessageV0 {
            nonce: self.nonce(),
            domain: Domain::UpdatePrice as u32,
            parameters: args,
        });
        let auth =
            Authorization::Ed25519(admin.sign(msg).unwrap().try_into_val(&self.env).unwrap());
        updt_price(&mut self.env, &self.contract_id, &auth, &n, &d)
    }

    pub fn get_price(&mut self) -> Price {
        get_price(&mut self.env, &self.contract_id)
    }
}
