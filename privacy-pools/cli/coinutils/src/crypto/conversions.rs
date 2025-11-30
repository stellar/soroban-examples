use soroban_sdk::{crypto::bls12_381::Fr as BlsScalar, Env, BytesN, U256};
use num_bigint::BigUint;
use crate::error::{CoinUtilsError, Result};

/// Convert a decimal string to a BlsScalar
pub fn decimal_string_to_bls_scalar(env: &Env, decimal_str: &str) -> Result<BlsScalar> {
    // For now, let's use a simpler approach that works with the existing system
    // We'll convert the decimal to a u128 first, then to BlsScalar
    if let Ok(value) = decimal_str.parse::<u128>() {
        // Convert u128 to BlsScalar
        return Ok(BlsScalar::from_u256(U256::from_u32(env, value as u32)));
    }
    
    // For very large numbers, we need to handle them differently
    // Since the decimal numbers are too large for u128, we'll use a workaround
    // by converting through the existing hex conversion system
    
    // First, let's try to convert the decimal to hex manually
    let mut temp = decimal_str.to_string();
    let mut hex_digits = String::new();
    
    while !temp.is_empty() && temp != "0" {
        let mut carry = 0u32;
        let mut new_temp = String::new();
        
        for ch in temp.chars() {
            let digit = ch.to_digit(10).ok_or_else(|| CoinUtilsError::InvalidDecimalCharacter(ch))? as u32;
            let value = carry * 10 + digit;
            new_temp.push((b'0' + (value / 16) as u8) as char);
            carry = value % 16;
        }
        
        // Remove leading zeros
        while new_temp.len() > 1 && new_temp.starts_with('0') {
            new_temp.remove(0);
        }
        
        if new_temp.is_empty() {
            new_temp = "0".to_string();
        }
        
        temp = new_temp;
        hex_digits.push_str(&format!("{:x}", carry));
    }
    
    // Reverse the hex string since we built it backwards
    let hex_str: String = hex_digits.chars().rev().collect();
    
    // Pad to 64 hex characters (32 bytes)
    let padded_hex = format!("{:0>64}", hex_str);
    
    // Convert hex to bytes
    let bytes = hex::decode(&padded_hex)
        .map_err(|e| CoinUtilsError::Hex(e))?;
    
    if bytes.len() != 32 {
        return Err(CoinUtilsError::InvalidByteLength(bytes.len()));
    }
    
    let mut byte_array = [0u8; 32];
    byte_array.copy_from_slice(&bytes);
    
    Ok(BlsScalar::from_bytes(BytesN::from_array(env, &byte_array)))
}

/// Convert BlsScalar to decimal string
pub fn bls_scalar_to_decimal_string(scalar: &BlsScalar) -> String {
    let array = scalar.to_bytes().to_array();
    bytes_to_decimal_string(&array)
}

/// Convert bytes to decimal string using num-bigint for efficient conversion
pub fn bytes_to_decimal_string(bytes: &[u8; 32]) -> String {
    let biguint = BigUint::from_bytes_be(bytes);
    biguint.to_str_radix(10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;
    
    #[test]
    fn test_decimal_to_bls_scalar_conversion() {
        let env = Env::default();
        let decimal_str = "123456789";
        let result = decimal_string_to_bls_scalar(&env, decimal_str);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_bls_scalar_to_decimal_conversion() {
        let env = Env::default();
        let scalar = BlsScalar::from_u256(U256::from_u32(&env, 123456789));
        let result = bls_scalar_to_decimal_string(&scalar);
        assert_eq!(result, "123456789");
    }
    
    #[test]
    fn test_invalid_decimal_character() {
        let env = Env::default();
        let decimal_str = "123abc456";
        let result = decimal_string_to_bls_scalar(&env, decimal_str);
        assert!(result.is_err());
    }
}
