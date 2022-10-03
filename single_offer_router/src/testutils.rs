#![cfg(any(test, feature = "testutils"))]
use ed25519_dalek::Keypair;
use soroban_auth::{Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{BigInt, BytesN, Env, IntoVal, Symbol};

use crate::SingleOfferRouterClient;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::SingleOfferRouter {});
}

fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

pub struct SingleOfferRouter {
    env: Env,
    contract_id: BytesN<32>,
}

impl SingleOfferRouter {
    fn client(&self) -> SingleOfferRouterClient {
        SingleOfferRouterClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }

    pub fn init(&self, admin: &Identifier, token_a: &[u8; 32], token_b: &[u8; 32], n: u32, d: u32) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);
        self.client().init(&admin, &token_a, &token_b, &n, &d)
    }

    pub fn safe_trade(&self, to: &Keypair, offer: &[u8; 32], amount: &BigInt, min: &BigInt) {
        let to_id = to_ed25519(&self.env, &to);
        let nonce = self.nonce(&to_id);
        let offer_addr = BytesN::from_array(&self.env, offer);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: Symbol::from_str("safe_trade"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (to_id, &nonce, &offer_addr, amount, min).into_val(&self.env),
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: to.public.to_bytes().into_val(&self.env),
            signature: to.sign(msg).unwrap().into_val(&self.env),
        });

        self.client()
            .safe_trade(&auth, &nonce, &offer_addr, &amount, &min)
    }

    pub fn get_offer(
        &self,
        admin: &Identifier,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
    ) -> BytesN<32> {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);
        self.client().get_offer(&admin, &token_a, &token_b)
    }

    pub fn nonce(&self, id: &Identifier) -> BigInt {
        self.client().nonce(&id)
    }
}
