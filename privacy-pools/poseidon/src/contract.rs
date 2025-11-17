use soroban_sdk::{
    contract, contractimpl, crypto::bls12_381::Fr, symbol_short, BytesN, Env,
};

use crate::Poseidon255;

const INNER: soroban_sdk::Symbol = symbol_short!("INNER");

#[contract]
pub struct PoseidonContract;

#[contractimpl]
impl PoseidonContract {
    /// Constructor to initialize the contract with a specific t value for Poseidon hash
    pub fn __constructor(env: Env, t: u32) {
        let inner = Poseidon255::new(&env, t as usize);
        env.storage().instance().set(&INNER, &inner);
    }

    /// Get a Poseidon instance using the stored t value
    fn get_poseidon(env: &Env) -> Poseidon255 {
        env.storage().instance().get(&INNER).unwrap()
    }

    /// Hash a single BLS12-381 scalar field element using Poseidon hash
    ///
    /// # Arguments
    /// * `input` - 32-byte input (as BytesN<32>)
    ///
    /// # Returns
    /// * 32-byte hash result (as BytesN<32>)
    pub fn hash(env: Env, input: BytesN<32>) -> BytesN<32> {
        let scalar = Fr::from_bytes(input);

        // Get Poseidon instance from stored configuration
        let poseidon = Self::get_poseidon(&env);

        // Compute hash
        let result = poseidon.hash(&env, &scalar);

        // Convert result back to bytes
        result.to_bytes()
    }

    /// Hash two BLS12-381 scalar field elements using Poseidon hash
    ///
    /// # Arguments
    /// * `input1` - First 32-byte input (as BytesN<32>)
    /// * `input2` - Second 32-byte input (as BytesN<32>)
    ///
    /// # Returns
    /// * 32-byte hash result (as BytesN<32>)
    pub fn hash_two(env: Env, input1: BytesN<32>, input2: BytesN<32>) -> BytesN<32> {
        // Convert input bytes to Fr
        let scalar1 = Fr::from_bytes(input1);
        let scalar2 = Fr::from_bytes(input2);

        // Get Poseidon instance from stored configuration
        let poseidon = Self::get_poseidon(&env);

        // Compute hash
        let result = poseidon.hash_two(&env, &scalar1, &scalar2);

        // Convert result back to bytes
        result.to_bytes()
    }
}

#[cfg(test)]
mod poseidon_contract_wasm {
    soroban_sdk::contractimport!(file = "opt/poseidon.wasm");
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::U256;

    #[test]
    fn test_contract_hash() {
        extern crate std;

        let env = Env::default();
        let contract_id = env.register(poseidon_contract_wasm::WASM, (2u32,));
        let client = poseidon_contract_wasm::Client::new(&env, &contract_id);

        // Create test input (32 bytes)
        let input_scalar = Fr::from_u256(U256::from_u32(&env, 123));
        let input = input_scalar.to_bytes();

        // Call contract
        let _result = client.hash(&input);
        // env.cost_estimate().budget().print();
        std::eprintln!("{:?}", env.cost_estimate().budget());
    }

    #[test]
    fn test_contract_hash_two() {
        extern crate std;

        let env = Env::default();
        let contract_id = env.register(poseidon_contract_wasm::WASM, (3u32,));
        let client = poseidon_contract_wasm::Client::new(&env, &contract_id);

        // Create test inputs (32 bytes each)
        let input1_scalar = Fr::from_u256(U256::from_u32(&env, 123));
        let input2_scalar = Fr::from_u256(U256::from_u32(&env, 456));

        let input1 = input1_scalar.to_bytes();
        let input2 = input2_scalar.to_bytes();

        // Call contract
        let _result = client.hash_two(&input1, &input2);
        // env.cost_estimate().budget().print();
        std::eprintln!("{:?}", env.cost_estimate().budget());
    }
}
