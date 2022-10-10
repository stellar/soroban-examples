#![allow(unused)]
use soroban_sdk::{Bytes, BytesN, Env};

soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");

pub fn create_contract(e: &Env, token_a: &BytesN<32>, token_b: &BytesN<32>) -> BytesN<32> {
    let mut salt = Bytes::new(&e);
    salt.append(&token_a.clone().into());
    salt.append(&token_b.clone().into());
    let salt = e.compute_hash_sha256(&salt);
    e.deployer().with_current_contract(salt).deploy_token()
}
