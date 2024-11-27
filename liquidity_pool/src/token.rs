#![allow(unused)]
use soroban_sdk::{symbol_short, xdr::ToXdr, Address, Bytes, BytesN, Env, FromVal, String, Symbol};

soroban_sdk::contractimport!(
    file = "target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
);

pub fn create_share_token(
    e: &Env,
    token_wasm_hash: BytesN<32>,
    token_a: &Address,
    token_b: &Address,
) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&token_a.to_xdr(e));
    salt.append(&token_b.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    e.deployer().with_current_contract(salt).deploy_v2(
        token_wasm_hash,
        (
            e.current_contract_address(),
            7u32,
            String::from_val(e, &"Pool Share Token"),
            String::from_val(e, &"POOL"),
        ),
    )
}
