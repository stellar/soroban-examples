#![cfg(any(test, feature = "testutils"))]
use ed25519_dalek::Keypair;
use soroban_sdk::{testutils::ed25519::Sign, BigInt, BytesN, Env, IntoVal, RawVal, Symbol, Vec};
use soroban_sdk_auth::{
    Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0,
};

use crate::LiquidityPoolRouterClient;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::LiquidityPoolRouter {});
}

fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
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
        to: &Keypair,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
        desired_a: &BigInt,
        min_a: &BigInt,
        desired_b: &BigInt,
        min_b: &BigInt,
    ) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        let to_id = to_ed25519(&self.env, &to);
        let nonce = self.nonce(&to_id);

        let mut args: Vec<RawVal> = Vec::new(&self.env);
        args.push_back(to_id.clone().into_val(&self.env));
        args.push_back(nonce.clone().into_val(&self.env));
        args.push_back(token_a.clone().into_val(&self.env));
        args.push_back(token_b.clone().into_val(&self.env));
        args.push_back(desired_a.clone().into_val(&self.env));
        args.push_back(min_a.clone().into_val(&self.env));
        args.push_back(desired_b.clone().into_val(&self.env));
        args.push_back(min_b.clone().into_val(&self.env));

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            function: Symbol::from_str("sf_deposit"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args,
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: to.public.to_bytes().into_val(&self.env),
            signature: to.sign(msg).unwrap().into_val(&self.env),
        });

        self.client().sf_deposit(
            &auth, &nonce, &token_a, &token_b, &desired_a, &min_a, &desired_b, &min_b,
        )
    }

    pub fn swap_out(
        &self,
        to: &Keypair,
        sell: &[u8; 32],
        buy: &[u8; 32],
        out: &BigInt,
        in_max: &BigInt,
    ) {
        let sell = BytesN::from_array(&self.env, sell);
        let buy = BytesN::from_array(&self.env, buy);

        let to_id = to_ed25519(&self.env, &to);
        let nonce = self.nonce(&to_id);

        let mut args: Vec<RawVal> = Vec::new(&self.env);
        args.push_back(to_id.clone().into_val(&self.env));
        args.push_back(nonce.clone().into_val(&self.env));
        args.push_back(sell.clone().into_val(&self.env));
        args.push_back(buy.clone().into_val(&self.env));
        args.push_back(out.clone().into_val(&self.env));
        args.push_back(in_max.clone().into_val(&self.env));

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            function: Symbol::from_str("swap_out"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args,
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: to.public.to_bytes().into_val(&self.env),
            signature: to.sign(msg).unwrap().into_val(&self.env),
        });

        self.client()
            .swap_out(&auth, &nonce, &sell, &buy, &out, &in_max)
    }

    pub fn sf_withdrw(
        &self,
        to: &Keypair,
        token_a: &[u8; 32],
        token_b: &[u8; 32],
        share_amount: &BigInt,
        min_a: &BigInt,
        min_b: &BigInt,
    ) {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        let to_id = to_ed25519(&self.env, &to);
        let nonce = self.nonce(&to_id);

        let mut args: Vec<RawVal> = Vec::new(&self.env);
        args.push_back(to_id.clone().into_val(&self.env));
        args.push_back(nonce.clone().into_val(&self.env));
        args.push_back(token_a.clone().into_val(&self.env));
        args.push_back(token_b.clone().into_val(&self.env));
        args.push_back(share_amount.clone().into_val(&self.env));
        args.push_back(min_a.clone().into_val(&self.env));
        args.push_back(min_b.clone().into_val(&self.env));

        let msg = SignaturePayload::V0(SignaturePayloadV0 {
            function: Symbol::from_str("sf_withdrw"),
            contract: self.contract_id.clone(),
            network: self.env.ledger().network_passphrase(),
            args,
        });
        let auth = Signature::Ed25519(Ed25519Signature {
            public_key: to.public.to_bytes().into_val(&self.env),
            signature: to.sign(msg).unwrap().into_val(&self.env),
        });

        self.client().sf_withdrw(
            &auth,
            &nonce,
            &token_a,
            &token_b,
            &share_amount,
            &min_a,
            &min_b,
        )
    }

    pub fn get_pool(&self, token_a: &[u8; 32], token_b: &[u8; 32]) -> BytesN<32> {
        let token_a = BytesN::from_array(&self.env, token_a);
        let token_b = BytesN::from_array(&self.env, token_b);

        self.client().get_pool(&token_a, &token_b)
    }

    pub fn nonce(&self, id: &Identifier) -> BigInt {
        self.client().nonce(&id)
    }
}
