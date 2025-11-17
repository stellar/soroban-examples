use crate::Poseidon255;
use soroban_sdk::{Env, crypto::bls12_381::Fr as BlsScalar, U256};

#[test]
fn test_hash_two_basic() {
    let env = Env::default();
    let poseidon = Poseidon255::new(&env, 3);
    let input1 = BlsScalar::from_u256(U256::from_u32(&env, 123));
    let input2 = BlsScalar::from_u256(U256::from_u32(&env, 456));
    
    let result = poseidon.hash_two(&env, &input1, &input2);
    
    // Verify that the result is not zero and is different from inputs
    let zero = BlsScalar::from_u256(U256::from_u32(&env, 0));
    assert_ne!(result, zero);
    assert_ne!(result, input1);
    assert_ne!(result, input2);
    
    // Verify that the same inputs produce the same result
    let result2 = poseidon.hash_two(&env, &input1, &input2);
    // env.cost_estimate().budget().print();
    assert_eq!(result, result2);
}

#[test]
fn test_hash_two_different_inputs() {
    let env = Env::default();
    let poseidon = Poseidon255::new(&env, 3);
    let input1 = BlsScalar::from_u256(U256::from_u32(&env, 123));
    let input2 = BlsScalar::from_u256(U256::from_u32(&env, 456));
    let input3 = BlsScalar::from_u256(U256::from_u32(&env, 789));
    
    let result1 = poseidon.hash_two(&env, &input1, &input2);
    let result2 = poseidon.hash_two(&env, &input1, &input3);
    
    // Different inputs should produce different results
    assert_ne!(result1, result2);
}

#[test]
fn test_hash_two_t3_constants() {
    let env = Env::default();
    let poseidon_t3 = Poseidon255::new(&env, 3);
    let input1 = BlsScalar::from_u256(U256::from_u32(&env, 123));
    let input2 = BlsScalar::from_u256(U256::from_u32(&env, 456));
    
    let result = poseidon_t3.hash_two(&env, &input1, &input2);
    
    // Verify that the result is not zero
    let zero = BlsScalar::from_u256(U256::from_u32(&env, 0));
    assert_ne!(result, zero);
}
