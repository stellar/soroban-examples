use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::{
    types::{GeneratedCoin, StateFile, AssociationSetFile},
    error::{CoinUtilsError, Result},
};

/// File manager for handling file I/O operations
pub struct FileManager;

impl FileManager {
    pub fn new() -> Self {
        Self
    }
    
    /// Read a coin file from disk
    pub fn read_coin_file(&self, path: &str) -> Result<GeneratedCoin> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CoinUtilsError::Io(e))?;
        serde_json::from_str(&content)
            .map_err(|e| CoinUtilsError::Json(e))
    }
    
    /// Write a coin file to disk
    pub fn write_coin_file(&self, coin: &GeneratedCoin, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(coin)
            .map_err(|e| CoinUtilsError::Json(e))?;
        let mut file = File::create(path)
            .map_err(|e| CoinUtilsError::Io(e))?;
        file.write_all(json.as_bytes())
            .map_err(|e| CoinUtilsError::Io(e))?;
        Ok(())
    }
    
    /// Read a state file from disk
    pub fn read_state_file(&self, path: &str) -> Result<StateFile> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CoinUtilsError::Io(e))?;
        serde_json::from_str(&content)
            .map_err(|e| CoinUtilsError::Json(e))
    }
    
    /// Write a state file to disk
    pub fn write_state_file(&self, state: &StateFile, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(state)
            .map_err(|e| CoinUtilsError::Json(e))?;
        let mut file = File::create(path)
            .map_err(|e| CoinUtilsError::Io(e))?;
        file.write_all(json.as_bytes())
            .map_err(|e| CoinUtilsError::Io(e))?;
        Ok(())
    }
    
    /// Read an association set file from disk
    pub fn read_association_file(&self, path: &str) -> Result<AssociationSetFile> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CoinUtilsError::Io(e))?;
        serde_json::from_str(&content)
            .map_err(|e| CoinUtilsError::Json(e))
    }
    
    /// Write an association set file to disk
    pub fn write_association_file(&self, association: &AssociationSetFile, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(association)
            .map_err(|e| CoinUtilsError::Json(e))?;
        let mut file = File::create(path)
            .map_err(|e| CoinUtilsError::Io(e))?;
        file.write_all(json.as_bytes())
            .map_err(|e| CoinUtilsError::Io(e))?;
        Ok(())
    }
    
    /// Check if a file exists
    pub fn file_exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }
    
    /// Create a new association set file if it doesn't exist
    pub fn create_association_file_if_not_exists(&self, path: &str) -> Result<AssociationSetFile> {
        if self.file_exists(path) {
            self.read_association_file(path)
        } else {
            let association = AssociationSetFile {
                labels: Vec::new(),
                scope: "default_scope".to_string(),
                root: None,
            };
            self.write_association_file(&association, path)?;
            Ok(association)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use crate::types::CoinData;
    
    #[test]
    fn test_file_operations() {
        let file_manager = FileManager::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        let coin = GeneratedCoin {
            coin: CoinData {
                value: "100".to_string(),
                nullifier: "200".to_string(),
                secret: "300".to_string(),
                label: "400".to_string(),
                commitment: "500".to_string(),
            },
            commitment_hex: "0x123".to_string(),
        };
        
        // Test write and read
        file_manager.write_coin_file(&coin, path).unwrap();
        let read_coin = file_manager.read_coin_file(path).unwrap();
        
        assert_eq!(coin.coin.value, read_coin.coin.value);
        assert_eq!(coin.commitment_hex, read_coin.commitment_hex);
    }
}
