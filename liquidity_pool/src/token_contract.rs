use stellar_contract_sdk::{Binary, Env};
use stellar_token_contract::public_types::U256;

#[cfg(not(feature = "testutils"))]
pub const TOKEN_CONTRACT: &[u8] = include_bytes!("../../wasm/stellar_token_contract.wasm");

#[cfg(not(feature = "testutils"))]
pub fn create_contract(e: &Env, token_a: &U256, token_b: &U256) -> Binary {
    let bin = Binary::from_slice(e, TOKEN_CONTRACT);
    let mut salt = Binary::new(&e);
    salt.append(&token_a.clone().into());
    salt.append(&token_b.clone().into());
    salt = e.compute_hash_sha256(salt);
    e.create_contract_from_contract(bin, salt).into()
}

#[cfg(feature = "testutils")]
use stellar_contract_sdk::IntoVal;
#[cfg(feature = "testutils")]
pub fn create_contract(e: &Env, token_a: &U256, token_b: &U256) -> Binary {
    use sha2::{Digest, Sha256};
    use std::vec::Vec;
    use stellar_xdr::{Hash, HashIdPreimage, HashIdPreimageContractId, Uint256, WriteXdr};

    let salt = {
        let mut salt_bin = Binary::new(&e);
        salt_bin.append(&token_a.clone().into());
        salt_bin.append(&token_b.clone().into());
        salt_bin = e.compute_hash_sha256(salt_bin);
        let mut salt_bytes: [u8; 32] = Default::default();
        for i in 0..salt_bin.len() {
            salt_bytes[i as usize] = salt_bin.get(i).unwrap();
        }
        Uint256(salt_bytes)
    };

    let contract_id = {
        let contract_id_bin = e.get_current_contract();
        let mut contract_id_bytes: [u8; 32] = Default::default();
        for i in 0..contract_id_bin.len() {
            contract_id_bytes[i as usize] = contract_id_bin.get(i).unwrap();
        }
        Hash(contract_id_bytes)
    };

    let new_contract_id = {
        let pre_image =
            HashIdPreimage::ContractIdFromContract(HashIdPreimageContractId { contract_id, salt });
        let mut buf = Vec::new();
        pre_image.write_xdr(&mut buf).unwrap();
        Sha256::digest(buf).into_val(e)
    };

    stellar_token_contract::testutils::register_test_contract(e, &new_contract_id);
    Binary::from_array(e, new_contract_id)
}
