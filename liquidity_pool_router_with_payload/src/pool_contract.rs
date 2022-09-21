use soroban_sdk::{BytesN, Env};

soroban_sdk::contractimport!(
    file = "../target/wasm32-unknown-unknown/release/soroban_liquidity_pool_contract.wasm"
);
pub type Client = ContractClient;

pub fn create_contract(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    use soroban_sdk::Bytes;
    let bin = Bytes::from_slice(e, WASM);
    e.deployer().from_current_contract(salt).deploy(bin)
}
