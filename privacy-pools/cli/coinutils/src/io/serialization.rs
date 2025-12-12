use crate::{
    error::{CoinUtilsError, Result},
    types::SnarkInput,
};

/// Serialization utilities for different data formats
pub struct SerializationManager;

impl SerializationManager {
    pub fn new() -> Self {
        Self
    }

    /// Serialize SNARK input to JSON string
    pub fn serialize_snark_input(&self, input: &SnarkInput) -> Result<String> {
        serde_json::to_string_pretty(input).map_err(|e| CoinUtilsError::Json(e))
    }

    /// Deserialize JSON string to SNARK input
    pub fn deserialize_snark_input(&self, json: &str) -> Result<SnarkInput> {
        serde_json::from_str(json).map_err(|e| CoinUtilsError::Json(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SnarkInput;

    #[test]
    fn test_snark_input_serialization() {
        let manager = SerializationManager::new();
        let input = SnarkInput {
            withdrawn_value: "1000".to_string(),
            label: "2000".to_string(),
            value: "3000".to_string(),
            nullifier: "4000".to_string(),
            secret: "5000".to_string(),
            state_root: "6000".to_string(),
            state_index: "0".to_string(),
            state_siblings: vec!["7000".to_string(), "8000".to_string()],
            association_root: "9000".to_string(),
            label_index: "1".to_string(),
            label_siblings: vec!["10000".to_string(), "11000".to_string()],
        };

        let json = manager.serialize_snark_input(&input).unwrap();
        let deserialized = manager.deserialize_snark_input(&json).unwrap();

        assert_eq!(input.withdrawn_value, deserialized.withdrawn_value);
        assert_eq!(input.label, deserialized.label);
        assert_eq!(
            input.state_siblings.len(),
            deserialized.state_siblings.len()
        );
    }
}
