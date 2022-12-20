#![cfg(any(test, feature = "testutils"))]

use crate::contract::TokenClient;
use ed25519_dalek::Keypair;
use soroban_auth::{Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{symbol, Bytes, BytesN, Env, IntoVal};

pub fn register_test_contract(e: &Env) -> BytesN<32> {
    e.register_contract(None, crate::contract::Token {})
}

pub fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

pub struct Token {
    env: Env,
    contract_id: BytesN<32>,
}

impl Token {
    pub fn new(env: &Env, contract_id: &BytesN<32>) -> Self {
        Self {
            env: env.clone(),
            contract_id: contract_id.clone(),
        }
    }

    pub fn initialize(&self, admin: &Identifier, decimals: u32, name: &str, symbol: &str) {
        let name: Bytes = name.into_val(&self.env);
        let symbol: Bytes = symbol.into_val(&self.env);
        TokenClient::new(&self.env, &self.contract_id).initialize(admin, &decimals, &name, &symbol);
    }

    pub fn nonce(&self, id: &Identifier) -> i128 {
        TokenClient::new(&self.env, &self.contract_id).nonce(id)
    }

    pub fn allowance(&self, from: &Identifier, spender: &Identifier) -> i128 {
        TokenClient::new(&self.env, &self.contract_id).allowance(from, spender)
    }

    pub fn incr_allow(&self, from: &Keypair, spender: &Identifier, amount: &i128) {
        let from_id = to_ed25519(&self.env, from);
        let nonce = self.nonce(&from_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("incr_allow"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (from_id, &nonce, spender, amount).into_val(&self.env),
        });

        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: from.public.to_bytes().into_val(&self.env),
            signature: from.sign(msg).unwrap().into_val(&self.env),
        });
        TokenClient::new(&self.env, &self.contract_id).incr_allow(&auth, &nonce, spender, amount)
    }

    pub fn decr_allow(&self, from: &Keypair, spender: &Identifier, amount: &i128) {
        let from_id = to_ed25519(&self.env, from);
        let nonce = self.nonce(&from_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("decr_allow"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (from_id, &nonce, spender, amount).into_val(&self.env),
        });

        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: from.public.to_bytes().into_val(&self.env),
            signature: from.sign(msg).unwrap().into_val(&self.env),
        });
        TokenClient::new(&self.env, &self.contract_id).decr_allow(&auth, &nonce, spender, amount)
    }

    pub fn balance(&self, id: &Identifier) -> i128 {
        TokenClient::new(&self.env, &self.contract_id).balance(id)
    }

    pub fn authorized(&self, id: &Identifier) -> bool {
        TokenClient::new(&self.env, &self.contract_id).authorized(id)
    }

    pub fn xfer(&self, from: &Keypair, to: &Identifier, amount: &i128) {
        let from_id = to_ed25519(&self.env, from);
        let nonce = self.nonce(&from_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("xfer"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (from_id, &nonce, to, amount).into_val(&self.env),
        });

        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: BytesN::from_array(&self.env, &from.public.to_bytes()),
            signature: from.sign(msg).unwrap().into_val(&self.env),
        });

        TokenClient::new(&self.env, &self.contract_id).xfer(&auth, &nonce, to, amount)
    }

    pub fn xfer_from(&self, spender: &Keypair, from: &Identifier, to: &Identifier, amount: &i128) {
        let spender_id = to_ed25519(&self.env, spender);
        let nonce = self.nonce(&spender_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("xfer_from"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (spender_id, &nonce, from, to, amount).into_val(&self.env),
        });

        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: spender.public.to_bytes().into_val(&self.env),
            signature: spender.sign(msg).unwrap().into_val(&self.env),
        });

        TokenClient::new(&self.env, &self.contract_id).xfer_from(&auth, &nonce, from, to, amount)
    }

    pub fn burn(&self, from: &Keypair, amount: &i128) {
        let from_id = to_ed25519(&self.env, from);
        let nonce = self.nonce(&from_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("burn"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (from_id, &nonce, amount).into_val(&self.env),
        });

        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: BytesN::from_array(&self.env, &from.public.to_bytes()),
            signature: from.sign(msg).unwrap().into_val(&self.env),
        });

        TokenClient::new(&self.env, &self.contract_id).burn(&auth, &nonce, amount)
    }

    pub fn burn_from(&self, spender: &Keypair, from: &Identifier, amount: &i128) {
        let spender_id = to_ed25519(&self.env, spender);
        let nonce = self.nonce(&spender_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("burn_from"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (spender_id, &nonce, from, amount).into_val(&self.env),
        });

        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: spender.public.to_bytes().into_val(&self.env),
            signature: spender.sign(msg).unwrap().into_val(&self.env),
        });

        TokenClient::new(&self.env, &self.contract_id).burn_from(&auth, &nonce, from, amount)
    }

    pub fn clawback(&self, admin: &Keypair, from: &Identifier, amount: &i128) {
        let admin_id = to_ed25519(&self.env, admin);
        let nonce = self.nonce(&admin_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("clawback"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (admin_id, &nonce, from, amount).into_val(&self.env),
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: admin.public.to_bytes().into_val(&self.env),
            signature: admin.sign(msg).unwrap().into_val(&self.env),
        });
        TokenClient::new(&self.env, &self.contract_id).clawback(&auth, &nonce, from, amount)
    }

    pub fn set_auth(&self, admin: &Keypair, id: &Identifier, authorize: bool) {
        let admin_id = to_ed25519(&self.env, admin);
        let nonce = self.nonce(&admin_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("set_auth"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (admin_id, &nonce, id, authorize).into_val(&self.env),
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: admin.public.to_bytes().into_val(&self.env),
            signature: admin.sign(msg).unwrap().into_val(&self.env),
        });
        TokenClient::new(&self.env, &self.contract_id).set_auth(&auth, &nonce, id, &authorize)
    }

    pub fn mint(&self, admin: &Keypair, to: &Identifier, amount: &i128) {
        let admin_id = to_ed25519(&self.env, admin);
        let nonce = self.nonce(&admin_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("mint"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (admin_id, &nonce, to, amount).into_val(&self.env),
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: admin.public.to_bytes().into_val(&self.env),
            signature: admin.sign(msg).unwrap().into_val(&self.env),
        });
        TokenClient::new(&self.env, &self.contract_id).mint(&auth, &nonce, to, amount)
    }

    pub fn set_admin(&self, admin: &Keypair, new_admin: &Identifier) {
        let admin_id = to_ed25519(&self.env, admin);
        let nonce = self.nonce(&admin_id);

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            name: symbol!("set_admin"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args: (admin_id, &nonce, new_admin).into_val(&self.env),
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: admin.public.to_bytes().into_val(&self.env),
            signature: admin.sign(msg).unwrap().into_val(&self.env),
        });
        TokenClient::new(&self.env, &self.contract_id).set_admin(&auth, &nonce, new_admin)
    }

    pub fn decimals(&self) -> u32 {
        TokenClient::new(&self.env, &self.contract_id).decimals()
    }

    pub fn name(&self) -> Bytes {
        TokenClient::new(&self.env, &self.contract_id).name()
    }

    pub fn symbol(&self) -> Bytes {
        TokenClient::new(&self.env, &self.contract_id).symbol()
    }
}
