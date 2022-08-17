use soroban_sdk::{BytesN, Env};
use soroban_token_contract::public_types::U256;

#[cfg(any(not(feature = "testutils"), feature = "offer-wasm"))]
pub const POOL_CONTRACT: &[u8] = include_bytes!("../../soroban_liquidity_pool_contract.wasm");

#[cfg(any(not(feature = "testutils"), feature = "offer-wasm"))]
pub fn create_contract(e: &Env, salt: &U256) -> BytesN<32> {
    use soroban_sdk::Bytes;
    let bin = Bytes::from_slice(e, POOL_CONTRACT);
    e.create_contract_from_contract(bin, salt.clone())
}

#[cfg(all(feature = "testutils", not(feature = "token-wasm")))]
pub fn create_contract(e: &Env, salt: &U256) -> BytesN<32> {
    use sha2::{Digest, Sha256};
    use soroban_sdk::IntoVal;
    use stellar_xdr::{Hash, HashIdPreimage, HashIdPreimageContractId, Uint256, WriteXdr};

    use std::vec::Vec;

    let contract_id = Hash(e.get_current_contract().into());
    let pre_image = HashIdPreimage::ContractIdFromContract(HashIdPreimageContractId {
        contract_id,
        salt: Uint256(salt.clone().into()),
    });
    let mut buf = Vec::new();
    pre_image.write_xdr(&mut buf).unwrap();
    let new_contract_id = Sha256::digest(buf).into_val(e);

    soroban_liquidity_pool_contract::testutils::register_test_contract(e, &new_contract_id);
    BytesN::from_array(e, new_contract_id)
}
