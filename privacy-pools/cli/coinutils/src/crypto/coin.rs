use soroban_sdk::{crypto::bls12_381::Fr as BlsScalar, Env, BytesN, U256};
use rand::{thread_rng, Rng};
use crate::{
    config::COIN_VALUE,
    crypto::{poseidon_hash, random_fr},
    types::{CoinData, GeneratedCoin},
};

/// Generate a label for a coin based on scope and nonce
pub fn generate_label(env: &Env, scope: &[u8], nonce: &[u8; 32]) -> BlsScalar {
    // Convert scope and nonce to field elements for Poseidon hashing
    let scope_fr = BlsScalar::from_bytes(BytesN::from_array(env, &{
        let mut bytes = [0u8; 32];
        let len = scope.len().min(32);
        bytes[..len].copy_from_slice(&scope[..len]);
        bytes
    }));
    let nonce_fr = BlsScalar::from_bytes(BytesN::from_array(env, nonce));
    
    // Hash using Poseidon
    poseidon_hash(env, &[scope_fr, nonce_fr])
}

/// Generate a commitment for a coin
pub fn generate_commitment(
    env: &Env, 
    value: BlsScalar, 
    label: BlsScalar, 
    nullifier: BlsScalar, 
    secret: BlsScalar
) -> BlsScalar {
    let precommitment = poseidon_hash(env, &[nullifier, secret]);
    poseidon_hash(env, &[value, label, precommitment])
}

/// Generate a complete coin with all necessary components
pub fn generate_coin(env: &Env, scope: &[u8]) -> GeneratedCoin {
    use crate::crypto::conversions::bls_scalar_to_decimal_string;
    
    let value = BlsScalar::from_u256(U256::from_u32(env, COIN_VALUE as u32));
    let nullifier = random_fr(env);
    let secret = random_fr(env);
    let nonce = thread_rng().gen::<[u8; 32]>();
    let label = generate_label(env, scope, &nonce);
    let commitment = generate_commitment(env, value.clone(), label.clone(), nullifier.clone(), secret.clone());

    let value_decimal = bls_scalar_to_decimal_string(&value);
    let nullifier_decimal = bls_scalar_to_decimal_string(&nullifier);
    let secret_decimal = bls_scalar_to_decimal_string(&secret);
    let label_decimal = bls_scalar_to_decimal_string(&label);
    let commitment_decimal = bls_scalar_to_decimal_string(&commitment);

    let coin_data = CoinData {
        value: value_decimal,
        nullifier: nullifier_decimal,
        secret: secret_decimal,
        label: label_decimal,
        commitment: commitment_decimal,
    };

    GeneratedCoin {
        coin: coin_data,
        commitment_hex: format!("0x{}", hex::encode(commitment.to_bytes().to_array())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_label() {
        let env = Env::default();
        let scope = b"test_scope";
        let nonce = [1u8; 32];
        let result = generate_label(&env, scope, &nonce);
        // Just verify it doesn't panic and returns a valid scalar
        assert!(result.to_bytes().to_array().iter().any(|&x| x != 0));
    }
    
    #[test]
    fn test_generate_commitment() {
        let env = Env::default();
        let value = BlsScalar::from_u256(U256::from_u32(&env, 100));
        let label = BlsScalar::from_u256(U256::from_u32(&env, 200));
        let nullifier = BlsScalar::from_u256(U256::from_u32(&env, 300));
        let secret = BlsScalar::from_u256(U256::from_u32(&env, 400));
        
        let result = generate_commitment(&env, value, label, nullifier, secret);
        // Just verify it doesn't panic and returns a valid scalar
        assert!(result.to_bytes().to_array().iter().any(|&x| x != 0));
    }
    
    #[test]
    fn test_generate_coin() {
        let env = Env::default();
        let scope = b"test_scope";
        let result = generate_coin(&env, scope);
        
        // Verify the coin has all required fields
        assert!(!result.coin.value.is_empty());
        assert!(!result.coin.nullifier.is_empty());
        assert!(!result.coin.secret.is_empty());
        assert!(!result.coin.label.is_empty());
        assert!(!result.coin.commitment.is_empty());
        assert!(result.commitment_hex.starts_with("0x"));
    }
}
