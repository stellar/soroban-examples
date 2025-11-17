use soroban_sdk::{
    crypto::bls12_381::Fr as BlsScalar,
    Env, U256, BytesN,
};
use poseidon::Poseidon255;
use serde::Deserialize;
use std::io::{self, Read};
use num_bigint::BigUint;

#[derive(Deserialize)]
struct Input {
    #[serde(rename = "in1")]
    in1_value: serde_json::Value,
    #[serde(rename = "in2")]
    in2_value: serde_json::Value,
}

fn bls_scalar_to_decimal(scalar: BlsScalar) -> String {
    // Convert soroban_sdk BlsScalar to decimal string
    // Get the U256 representation
    let u256_val = scalar.to_u256();
    
    // Convert U256 to bytes and then to BigUint
    let bytes = u256_val.to_be_bytes();
    let mut bytes_array = [0u8; 32];
    bytes.copy_into_slice(&mut bytes_array);
    let biguint = BigUint::from_bytes_be(&bytes_array);
    
    biguint.to_str_radix(10)
}

fn biguint_to_bls_scalar(env: &Env, biguint: &BigUint) -> BlsScalar {
    // Convert BigUint to bytes (big-endian)
    let bytes = biguint.to_bytes_be();
    
    // Pad to 32 bytes if necessary
    let mut padded_bytes = [0u8; 32];
    let start_idx = 32 - bytes.len().min(32);
    padded_bytes[start_idx..].copy_from_slice(&bytes[..bytes.len().min(32)]);
    
    // Convert to BlsScalar
    BlsScalar::from_bytes(BytesN::from_array(env, &padded_bytes))
}

fn main() {
    // Create soroban environment for testing
    let env = Env::default();
    
    // Read JSON input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    // Parse the JSON input
    let input_data: Input = serde_json::from_str(&input).expect("Failed to parse JSON");
    
    // Convert to BlsScalar and hash
    let input1_scalar = match input_data.in1_value {
        serde_json::Value::String(s) => {
            // Parse the large number as a BigUint first
            let big_num = BigUint::parse_bytes(s.as_bytes(), 10)
                .expect("Failed to parse string to BigUint");
            // Convert BigUint to BlsScalar
            biguint_to_bls_scalar(&env, &big_num)
        },
        serde_json::Value::Number(n) => {
            if let Some(u64_val) = n.as_u64() {
                BlsScalar::from_u256(U256::from_u32(&env, u64_val as u32))
            } else {
                // For numbers too large for u64
                let s = n.to_string();
                let big_num = BigUint::parse_bytes(s.as_bytes(), 10)
                    .expect("Failed to parse number to BigUint");
                biguint_to_bls_scalar(&env, &big_num)
            }
        },
        _ => panic!("Expected string or number for 'in1' field"),
    };
    
    let input2_scalar = match input_data.in2_value {
        serde_json::Value::String(s) => {
            // Parse the large number as a BigUint first
            let big_num = BigUint::parse_bytes(s.as_bytes(), 10)
                .expect("Failed to parse string to BigUint");
            // Convert BigUint to BlsScalar
            biguint_to_bls_scalar(&env, &big_num)
        },
        serde_json::Value::Number(n) => {
            if let Some(u64_val) = n.as_u64() {
                BlsScalar::from_u256(U256::from_u32(&env, u64_val as u32))
            } else {
                // For numbers too large for u64
                let s = n.to_string();
                let big_num = BigUint::parse_bytes(s.as_bytes(), 10)
                    .expect("Failed to parse number to BigUint");
                biguint_to_bls_scalar(&env, &big_num)
            }
        },
        _ => panic!("Expected string or number for 'in2' field"),
    };
    
    let poseidon1 = Poseidon255::new(&env, 2);
    let output1 = poseidon1.hash(&env, &input1_scalar);
    let decimal_output1 = bls_scalar_to_decimal(output1);

    let poseidon2 = Poseidon255::new(&env, 3);
    let output2 = poseidon2.hash_two(&env, &input1_scalar, &input2_scalar);
    let decimal_output2 = bls_scalar_to_decimal(output2);

    println!("{}", decimal_output1);
    println!("{}", decimal_output2);
}
