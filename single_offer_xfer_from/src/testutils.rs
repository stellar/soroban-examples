#![cfg(any(test, feature = "testutils"))]

use crate::{Price, SingleOfferXferFromClient};
use ed25519_dalek::Keypair;
use soroban_auth::{Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{BigInt, BytesN, Env, IntoVal, RawVal, Symbol, Vec};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::SingleOfferXferFrom {});
}

fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

pub struct SingleOfferXferFrom {
    env: Env,
    contract_id: BytesN<32>,
}

impl SingleOfferXferFrom {
    fn client(&self) -> SingleOfferXferFromClient {
        SingleOfferXferFromClient::new(&self.env, &self.contract_id)
    }

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

        self.client().initialize(&admin, &token_a, &token_b, &n, &d)
    }

    pub fn nonce(&self, id: &Identifier) -> BigInt {
        self.client().nonce(&id)
    }

    pub fn trade(&self, to: &Keypair, amount_to_sell: &BigInt, min: &BigInt) {
        let id = to_ed25519(&self.env, &to);
        let nonce = self.nonce(&id);

        let mut args: Vec<RawVal> = Vec::new(&self.env);
        args.push_back(id.clone().into_val(&self.env));
        args.push_back(nonce.clone().into_val(&self.env));
        args.push_back(amount_to_sell.clone().into_val(&self.env));
        args.push_back(min.clone().into_val(&self.env));
        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            function: Symbol::from_str("trade"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args,
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: to.public.to_bytes().into_val(&self.env),
            signature: to.sign(msg).unwrap().into_val(&self.env),
        });

        self.client().trade(&auth, &nonce, &amount_to_sell, &min)
    }

    pub fn updt_price(&self, admin: &Keypair, n: u32, d: u32) {
        let id = to_ed25519(&self.env, &admin);
        let nonce = self.nonce(&id);

        let mut args: Vec<RawVal> = Vec::new(&self.env);
        args.push_back(id.clone().into_val(&self.env));
        args.push_back(nonce.clone().into_val(&self.env));
        args.push_back(n.into_val(&self.env));
        args.push_back(d.into_val(&self.env));
        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            function: Symbol::from_str("updt_price"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args,
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: admin.public.to_bytes().into_val(&self.env),
            signature: admin.sign(msg).unwrap().into_val(&self.env),
        });

        self.client().updt_price(&auth, &nonce, &n, &d)
    }

    pub fn get_price(&self) -> Price {
        self.client().get_price()
    }
}
