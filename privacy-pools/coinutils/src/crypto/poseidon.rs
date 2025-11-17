use soroban_sdk::{crypto::bls12_381::Fr as BlsScalar, Env, U256};
use poseidon::Poseidon255;

/// Poseidon-based hash for field elements
pub fn poseidon_hash(env: &Env, inputs: &[BlsScalar]) -> BlsScalar {
    let poseidon1 = Poseidon255::new(env, 2);
    let poseidon2 = Poseidon255::new(env, 3);
    
    match inputs.len() {
        1 => poseidon1.hash(env, &inputs[0]),
        2 => poseidon2.hash_two(env, &inputs[0], &inputs[1]),
        _ => {
            // For more than 2 inputs, hash them sequentially
            let mut result = inputs[0].clone();
            for input in inputs.iter().skip(1) {
                result = poseidon2.hash_two(env, &result, input);
            }
            result
        }
    }
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
