#![cfg(test)]

use crate::public_types::{KeyedEd25519Signature, Message, MessageV0, U256};

use super::*;
use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_sdk::testutils::ed25519::Sign;

use soroban_sdk::{Env, EnvVal, FixedBinary, Vec};

pub fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn make_auth(e: &Env, contract_id: &U256, kp: &Keypair, data: &BigInt) -> KeyedAuthorization {
    let id = to_ed25519(&e, &kp);

    let mut args: Vec<EnvVal> = Vec::new(&e);
    args.push(data.clone().into_env_val(&e));
    let msg = Message::V0(MessageV0 {
        nonce: nonce::invoke(&e, &contract_id, &id),
        domain: cryptography::Domain::SaveData as u32,
        parameters: args,
    });
    KeyedAuthorization::Ed25519(KeyedEd25519Signature {
        public_key: FixedBinary::from_array(&e, kp.public.to_bytes()),
        signature: kp.sign(msg).unwrap().into_val(e),
    })
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, AuthContract);

    let user1 = generate_keypair();
    let data = BigInt::from_u32(&env, 2);

    let auth = make_auth(&env, &contract_id, &user1, &data);
    save_data::invoke(&env, &contract_id, &auth, &data)
}
