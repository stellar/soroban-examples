#![allow(unused)]
use soroban_sdk::{Bytes, BytesN, Env};
use soroban_token_contract::public_types::U256;

pub fn create_contract(e: &Env, token_a: &U256, token_b: &U256) -> BytesN<32> {
    let mut salt = Bytes::new(&e);
    salt.append(&token_a.clone().into());
    salt.append(&token_b.clone().into());
    let salt = e.compute_hash_sha256(salt);
    e.create_token_from_contract(salt)
}
