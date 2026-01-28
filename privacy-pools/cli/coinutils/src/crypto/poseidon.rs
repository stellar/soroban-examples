use soroban_poseidon::poseidon_hash as poseidon_hash_native;
use soroban_sdk::{crypto::bls12_381::Fr as BlsScalar, Env, Vec, U256};

/// Poseidon-based hash for field elements using native SDK implementation
/// Uses poseidon_hash (not poseidon2_hash) to match the circom circuit
pub fn poseidon_hash(env: &Env, inputs: &[BlsScalar]) -> BlsScalar {
    // Convert Fr inputs to U256
    let mut u256_inputs = Vec::new(env);
    for input in inputs.iter() {
        u256_inputs.push_back(BlsScalar::to_u256(input));
    }

    // Hash using native implementation with appropriate state size
    // State size t = inputs.len() + 1 (rate = t - 1 = inputs.len())
    let result_u256 = match inputs.len() {
        1 => poseidon_hash_native::<2, BlsScalar>(env, &u256_inputs),
        2 => poseidon_hash_native::<3, BlsScalar>(env, &u256_inputs),
        3 => poseidon_hash_native::<4, BlsScalar>(env, &u256_inputs),
        _ => panic!("poseidon_hash supports 1-3 inputs"),
    };

    // Convert U256 result back to Fr
    BlsScalar::from_u256(result_u256)
}

/// Generate a random field element
pub fn random_fr(env: &Env) -> BlsScalar {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    BlsScalar::from_u256(U256::from_u32(env, rng.gen::<u32>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_hash_single_input() {
        let env = Env::default();
        let input = BlsScalar::from_u256(U256::from_u32(&env, 123));
        let result = poseidon_hash(&env, &[input]);
        // Just verify it doesn't panic and returns a valid scalar
        assert!(result.to_bytes().to_array().iter().any(|&x| x != 0));
    }

    #[test]
    fn test_poseidon_hash_two_inputs() {
        let env = Env::default();
        let input1 = BlsScalar::from_u256(U256::from_u32(&env, 123));
        let input2 = BlsScalar::from_u256(U256::from_u32(&env, 456));
        let result = poseidon_hash(&env, &[input1, input2]);
        // Just verify it doesn't panic and returns a valid scalar
        assert!(result.to_bytes().to_array().iter().any(|&x| x != 0));
    }

    #[test]
    fn test_random_fr() {
        let env = Env::default();
        let result = random_fr(&env);
        // Just verify it doesn't panic and returns a valid scalar
        assert!(result.to_bytes().to_array().iter().any(|&x| x != 0));
    }
}
