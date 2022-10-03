#![cfg(any(test, feature = "testutils"))]

use crate::{Price, SingleOfferClient};
use ed25519_dalek::Keypair;
use soroban_auth::{Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{BigInt, BytesN, Env, IntoVal, Symbol};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::SingleOffer {});
}

pub fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

pub struct SingleOffer {
    env: Env,
    contract_id: BytesN<32>,
}

impl SingleOffer {
    fn client(&self) -> SingleOfferClient {
        SingleOfferClient::new(&self.env, &self.contract_id)
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
        self.client().initialize(&admin, &token_a, &token_b, &n, &d);
    }

    pub fn nonce(&self) -> BigInt {
        self.client().nonce()
    }

    pub fn trade(&self, to: &Identifier, min: &BigInt) {
        self.client().trade(&to, &min)
    }

    pub fn withdraw(&self, admin: &Keypair, amount: &BigInt) {
        let nonce = self.nonce();
        let admin_id = to_ed25519(&self.env, admin);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: Symbol::from_str("withdraw"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (admin_id, &nonce, amount).into_val(&self.env),
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: admin.public.to_bytes().into_val(&self.env),
            signature: admin.sign(msg).unwrap().into_val(&self.env),
        });

        self.client().withdraw(&auth, &nonce, &amount)
    }

    pub fn updt_price(&self, admin: &Keypair, n: u32, d: u32) {
        let nonce = self.nonce();
        let admin_id = to_ed25519(&self.env, admin);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: Symbol::from_str("updt_price"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (admin_id, &nonce, n, d).into_val(&self.env),
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

    pub fn get_sell(&self) -> BytesN<32> {
        self.client().get_sell()
    }

    pub fn get_buy(&self) -> BytesN<32> {
        self.client().get_buy()
    }
}
