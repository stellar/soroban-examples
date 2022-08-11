#![cfg(any(test, feature = "testutils"))]

use crate::cryptography::Domain;
use crate::Price;
use ed25519_dalek::Keypair;
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{BigInt, Env, EnvVal, FixedBinary, IntoVal, Vec};
use soroban_token_contract::public_types::{
    Authorization, Identifier, KeyedAuthorization, KeyedEd25519Signature, Message, MessageV0,
};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = FixedBinary::from_array(e, *contract_id);
    e.register_contract(&contract_id, crate::SingleOfferXferFrom {});
}

pub use crate::get_price::invoke as get_price;
pub use crate::initialize::invoke as initialize;
pub use crate::nonce::invoke as nonce;
pub use crate::trade::invoke as trade;
pub use crate::updt_price::invoke as updt_price;

pub struct SingleOfferXferFrom {
    env: Env,
    contract_id: FixedBinary<32>,
}

impl SingleOfferXferFrom {
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

    pub fn nonce(&self, id: &Identifier) -> BigInt {
        nonce(&self.env, &self.contract_id, id)
    }

    pub fn trade(&self, to: &Keypair, amount_to_sell: &BigInt, min: &BigInt) {
        let mut args: Vec<EnvVal> = Vec::new(&self.env);
        args.push(amount_to_sell.clone().into_env_val(&self.env));
        args.push(min.clone().into_env_val(&self.env));
        let msg = Message::V0(MessageV0 {
            nonce: self.nonce(&Identifier::Ed25519(
                to.public.to_bytes().into_val(&self.env),
            )),
            domain: Domain::Trade as u32,
            parameters: args,
        });
        let auth = KeyedAuthorization::Ed25519(KeyedEd25519Signature {
            public_key: to.public.to_bytes().into_val(&self.env),
            signature: to.sign(msg).unwrap().into_val(&self.env),
        });

        trade(&self.env, &self.contract_id, &auth, amount_to_sell, min)
    }

    pub fn updt_price(&self, admin: &Keypair, n: u32, d: u32) {
        let mut args: Vec<EnvVal> = Vec::new(&self.env);
        args.push(n.into_env_val(&self.env));
        args.push(d.into_env_val(&self.env));
        let msg = Message::V0(MessageV0 {
            nonce: self.nonce(&Identifier::Ed25519(
                admin.public.to_bytes().into_val(&self.env),
            )),
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
