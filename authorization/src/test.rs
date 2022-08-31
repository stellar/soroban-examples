#![cfg(test)]

use super::*;
use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_sdk::testutils::ed25519::Sign;

use soroban_sdk::{BytesN, Env, RawVal, Symbol, Vec};
use soroban_sdk_auth::{Ed25519Signature, SignaturePayload, SignaturePayloadV0};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn make_identifier(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

fn make_signature(e: &Env, kp: &Keypair, function: &str, args: Vec<RawVal>) -> Signature {
    let msg = SignaturePayload::V0(SignaturePayloadV0 {
        function: Symbol::from_str(function),
        contract: BytesN::from_array(e, &[0; 32]),
        network: e.ledger().network_passphrase(),
        args,
    });
    Signature::Ed25519(Ed25519Signature {
        public_key: BytesN::from_array(e, &kp.public.to_bytes()),
        signature: kp.sign(msg).unwrap().into_val(e),
    })
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ExampleContract);
    let client = ExampleContractClient::new(&env, contract_id);

    // 1. Initialize contract by setting the admin.
    let admin_kp = generate_keypair();
    let admin_id = make_identifier(&env, &admin_kp);
    client.set_admin(&admin_id);

    // 2. Store a num for user1.
    let user1_kp = generate_keypair();
    let user1_id = make_identifier(&env, &user1_kp);
    let num = BigInt::from_u32(&env, 2);

    let user1_nonce = client.nonce(&user1_id);

    let sig = make_signature(
        &env,
        &user1_kp,
        "save_num",
        (&user1_id, &user1_nonce, &num).into_val(&env),
    );
    client.save_num(&sig, &user1_nonce, &num);

    // 3. Overwrite user1's num using admin.
    let new_num = BigInt::from_u32(&env, 10);

    let admin_nonce = client.nonce(&admin_id);
    let sig = make_signature(
        &env,
        &admin_kp,
        "overwrite",
        (&admin_id, &admin_nonce, &user1_id, &new_num).into_val(&env),
    );

    client.overwrite(&sig, &admin_nonce, &user1_id, &new_num);
}

#[test]
#[should_panic(expected = "Failed ED25519 verification")]
fn bad_data() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ExampleContract);
    let client = ExampleContractClient::new(&env, contract_id);

    // 1. Sign arguments with user1's keypair.
    let user1_kp = generate_keypair();
    let user1_id = make_identifier(&env, &user1_kp);
    let signed_num = BigInt::from_u32(&env, 1);

    let nonce = client.nonce(&user1_id);

    let sig = make_signature(
        &env,
        &user1_kp,
        "save_data",
        (&user1_id, &nonce, &signed_num).into_val(&env),
    );

    // 2. Attempt to invoke with user1's signature, but with different
    // arguments. Expect panic.
    let bad_num = BigInt::from_u32(&env, 2);
    client.save_num(&sig, &nonce, &bad_num);
}
