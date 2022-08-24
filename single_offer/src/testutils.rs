#![cfg(any(test, feature = "testutils"))]

use crate::Price;
use ed25519_dalek::Keypair;
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{BigInt, BytesN, Env, IntoVal, RawVal, Symbol, Vec};
use soroban_sdk_auth::public_types::{Ed25519Signature, Identifier, Message, MessageV0, Signature};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
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
            contract_id: BytesN::from_array(env, contract_id),
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
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);
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
        let nonce = self.nonce();

        let mut args: Vec<RawVal> = Vec::new(&self.env);
        args.push(nonce.clone().into_val(&self.env));
        args.push(amount.clone().into_val(&self.env));
        let msg = Message::V0(MessageV0 {
            function: Symbol::from_str("withdraw"),
            contrct_id: self.contract_id.clone(),
            network_id: self.env.ledger().network_passphrase(),
            args,
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: admin.public.to_bytes().into_val(&self.env),
            signature: admin.sign(msg).unwrap().into_val(&self.env),
        });
        crate::withdraw(&self.env, &self.contract_id, &auth, &nonce, amount)
    }

    pub fn updt_price(&self, admin: &Keypair, n: u32, d: u32) {
        let nonce = self.nonce();

        let mut args: Vec<RawVal> = Vec::new(&self.env);
        args.push(nonce.clone().into_val(&self.env));
        args.push(n.into_val(&self.env));
        args.push(d.into_val(&self.env));
        let msg = Message::V0(MessageV0 {
            function: Symbol::from_str("updt_price"),
            contrct_id: self.contract_id.clone(),
            network_id: self.env.ledger().network_passphrase(),
            args,
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: admin.public.to_bytes().into_val(&self.env),
            signature: admin.sign(msg).unwrap().into_val(&self.env),
        });
        crate::updt_price(&self.env, &self.contract_id, &auth, &nonce, &n, &d)
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
