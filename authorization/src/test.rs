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

fn make_auth(e: &Env, kp: &Keypair, args: Vec<EnvVal>, function: &str) -> KeyedAuthorization {
    let msg = Message::V0(MessageV0 {
        function: Symbol::from_str(function),
        contrct_id: BytesN::from_array(&e, [0; 32]),
        network_id: e.ledger().network_passphrase(),
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

    // 1. set the admin
    let admin = generate_keypair();
    set_admin::invoke(&env, &contract_id, &to_ed25519(&env, &admin));

    // 2. store data with user1's auth
    let user1 = generate_keypair();
    let user1_id = &to_ed25519(&env, &user1);
    let data = BigInt::from_u32(&env, 2);

    let user1_nonce = nonce::invoke(&env, &contract_id, user1_id);

    let mut args: Vec<EnvVal> = Vec::new(&env);
    args.push(user1_nonce.clone().into_env_val(&env));
    args.push(data.clone().into_env_val(&env));

    let auth = make_auth(&env, &user1, args, "save_data");
    save_data::invoke(&env, &contract_id, &auth, &user1_nonce, &data);

    // 3. Overwrite user1's data using admin
    let new_data = BigInt::from_u32(&env, 10);

    let admin_nonce = nonce::invoke(&env, &contract_id, &to_ed25519(&env, &admin));
    let mut args: Vec<EnvVal> = Vec::new(&env);
    args.push(admin_nonce.clone().into_env_val(&env));
    args.push(user1_id.clone().into_env_val(&env));
    args.push(new_data.clone().into_env_val(&env));

    let auth = make_auth(&env, &admin, args, "overwrite");

    overwrite::invoke(&env, &contract_id, &auth, &admin_nonce, user1_id, &new_data);
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
    let mut args: Vec<EnvVal> = Vec::new(&env);
    args.push(nonce.clone().into_env_val(&env));
    args.push(signed_data.clone().into_env_val(&env));

    let auth = make_auth(&env, &user1, args, "save_data");
    save_data::invoke(&env, &contract_id, &auth, &nonce, &data)
}
