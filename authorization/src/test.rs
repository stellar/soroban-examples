#![cfg(test)]

use crate::public_types::{KeyedEd25519Signature, Message, MessageV0};

use super::*;
use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_sdk::testutils::ed25519::Sign;

use soroban_sdk::{BytesN, Env, EnvVal, Vec};

pub fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn make_auth(e: &Env, kp: &Keypair, nonce: &BigInt, data: &BigInt) -> KeyedAuthorization {
    let mut args: Vec<EnvVal> = Vec::new(&e);
    args.push(nonce.clone().into_env_val(&e));
    args.push(data.clone().into_env_val(&e));

    let msg = Message::V0(MessageV0 {
        function: Symbol::from_str("save_data"),
        parameters: args,
    });
    KeyedAuthorization::Ed25519(KeyedEd25519Signature {
        public_key: BytesN::from_array(&e, kp.public.to_bytes()),
        signature: kp.sign(msg).unwrap().into_val(e),
    })
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, AuthContract);

    let user1 = generate_keypair();
    let data = BigInt::from_u32(&env, 2);

    let nonce = nonce::invoke(&env, &contract_id, &to_ed25519(&env, &user1));
    let auth = make_auth(&env, &user1, &nonce, &data);
    save_data::invoke(&env, &contract_id, &auth, &nonce, &data)
}

#[test]
#[should_panic(expected = "Failed ED25519 verification")]
fn bad_data() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, AuthContract);

    let user1 = generate_keypair();
    let signed_data = BigInt::from_u32(&env, 1);
    let data = BigInt::from_u32(&env, 2);

    let nonce = nonce::invoke(&env, &contract_id, &to_ed25519(&env, &user1));
    let auth = make_auth(&env, &user1, &nonce, &signed_data);
    save_data::invoke(&env, &contract_id, &auth, &nonce, &data)
}
