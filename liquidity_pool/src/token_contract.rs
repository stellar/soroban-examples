#![allow(unused)]
use soroban_sdk::{Bytes, BytesN, Env};

// Creating the token contract happens a couple different ways depending on the
// situation:
//
// In tests, or when imported with testutils, without the token-wasm
// feature, we use the imported token contract library and register it manually
// as a test contract.
//
// In tests, when token-wasm feature is enabled, we use the embedded token wasm
// file.
//
// Outside of tests and testutils, we use the embedded token wasm file.

soroban_sdk::contractimport!(
    file = "../target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
);
pub type TokenClient = Client;

#[cfg(not(all(any(test, feature = "testutils"), not(feature = "token-wasm"))))]
pub fn create_contract(e: &Env, token_a: &BytesN<32>, token_b: &BytesN<32>) -> BytesN<32> {
    let bin = Bytes::from_slice(e, WASM);
    let mut salt = Bytes::new(&e);
    salt.append(&token_a.clone().into());
    salt.append(&token_b.clone().into());
    let salt = e.compute_hash_sha256(&salt);
    e.deployer().with_current_contract(salt).deploy(bin)
}

#[cfg(all(any(test, feature = "testutils"), not(feature = "token-wasm")))]
extern crate std;

#[cfg(all(any(test, feature = "testutils"), not(feature = "token-wasm")))]
pub fn create_contract(e: &Env, token_a: &BytesN<32>, token_b: &BytesN<32>) -> BytesN<32> {
    use sha2::{Digest, Sha256};
    use soroban_sdk::IntoVal;
    use std::vec::Vec;
    use stellar_xdr::{Hash, HashIdPreimage, HashIdPreimageContractId, Uint256, WriteXdr};

    let salt = {
        let mut salt_bin = Bytes::new(&e);
        salt_bin.append(&token_a.clone().into());
        salt_bin.append(&token_b.clone().into());
        Uint256(e.compute_hash_sha256(&salt_bin).into())
    };

    let contract_id = Hash(e.get_current_contract().into());

    let pre_image =
        HashIdPreimage::ContractIdFromContract(HashIdPreimageContractId { contract_id, salt });

    let mut buf = Vec::new();
    pre_image.write_xdr(&mut buf).unwrap();
    let new_contract_id = Sha256::digest(buf).into();

    soroban_token_contract::testutils::register_test_contract(e, &new_contract_id);
    BytesN::from_array(e, &new_contract_id)
}
