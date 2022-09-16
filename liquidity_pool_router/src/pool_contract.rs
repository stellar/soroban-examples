use soroban_sdk::{BytesN, Env};

soroban_sdk::contractimport!(file = "../soroban_liquidity_pool_contract.wasm");
pub type LiquidityPoolClient = ContractClient;

#[cfg(not(all(any(test, feature = "testutils"), not(feature = "token-wasm"))))]
pub fn create_contract(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    use soroban_sdk::Bytes;
    let bin = Bytes::from_slice(e, WASM);
    e.deployer().from_current_contract(salt).deploy(bin)
}

#[cfg(all(any(test, feature = "testutils"), not(feature = "token-wasm")))]
pub fn create_contract(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    use sha2::{Digest, Sha256};
    use stellar_xdr::{Hash, HashIdPreimage, HashIdPreimageContractId, Uint256, WriteXdr};

    use std::vec::Vec;

    let contract_id = Hash(e.get_current_contract().into());
    let pre_image = HashIdPreimage::ContractIdFromContract(HashIdPreimageContractId {
        contract_id,
        salt: Uint256(salt.clone().into()),
    });
    let mut buf = Vec::new();
    pre_image.write_xdr(&mut buf).unwrap();
    let new_contract_id = Sha256::digest(buf).into();

    soroban_liquidity_pool_contract::testutils::register_test_contract(e, &new_contract_id);
    BytesN::from_array(e, &new_contract_id)
}
