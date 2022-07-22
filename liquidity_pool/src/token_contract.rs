use stellar_contract_sdk::{Binary, Env, VariableLengthBinary};
use stellar_token_contract::public_types::U256;

#[cfg(not(feature = "external"))]
pub const TOKEN_CONTRACT: &[u8] = include_bytes!("../../wasm/stellar_token_contract.wasm");

#[cfg(not(feature = "external"))]
pub fn create_contract(e: &Env, token_a: &U256, token_b: &U256) -> Binary {
    // TODO: Use linear memory for this
    let mut bin = Binary::new(&e);
    for x in crate::token_contract::TOKEN_CONTRACT {
        bin.push(*x);
    }
    let mut salt = Binary::new(&e);
    salt.append(&token_a.clone().into());
    salt.append(&token_b.clone().into());
    salt = e.compute_hash_sha256(salt);
    e.create_contract_from_contract(bin, salt).into()
}

#[cfg(feature = "external")]
pub fn create_contract(e: &Env, token_a: &U256, token_b: &U256) -> Binary {
    use sha2::{Digest, Sha256};
    use std::vec::Vec;
    use stellar_contract_sdk::FixedLengthBinary;
    use stellar_xdr::{Hash, HashIdPreimage, HashIdPreimageContractId, Uint256, WriteXdr};

    let salt = {
        let mut salt_bin = Binary::new(&e);
        salt_bin.append(&token_a.clone().into());
        salt_bin.append(&token_b.clone().into());
        salt_bin = e.compute_hash_sha256(salt_bin);
        let mut salt_bytes: [u8; 32] = Default::default();
        for i in 0..salt_bin.len() {
            salt_bytes[i as usize] = salt_bin.get(i);
        }
        Uint256(salt_bytes)
    };

    let contract_id = { Hash(Default::default()) };

    let new_contract_id = {
        let pre_image =
            HashIdPreimage::ContractIdFromContract(HashIdPreimageContractId { contract_id, salt });
        let mut buf = Vec::new();
        pre_image.write_xdr(&mut buf).unwrap();

        let mut id: [u8; 32] = Default::default();
        for (i, b) in Sha256::digest(buf).iter().enumerate() {
            id[i] = *b;
        }
        id
    };

    stellar_token_contract::external::register_test_contract(e, &new_contract_id);

    let mut res = Binary::new(e);
    for b in new_contract_id {
        res.push(b);
    }
    res
}
