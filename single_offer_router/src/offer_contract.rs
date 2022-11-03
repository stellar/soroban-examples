use soroban_sdk::{BytesN, Env};

soroban_sdk::contractimport!(
    file = "../target/wasm32-unknown-unknown/release/soroban_single_offer_contract.wasm"
);
pub type SingleOfferClient = Client;

pub fn create_contract(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    use soroban_sdk::Bytes;
    let bin = Bytes::from_slice(e, WASM);
    e.deployer().with_current_contract(salt).deploy(bin)
}
