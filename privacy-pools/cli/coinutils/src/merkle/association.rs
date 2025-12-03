use soroban_sdk::Env;
use lean_imt::LeanIMT;
use crate::{
    config::{ASSOCIATION_TREE_DEPTH, MAX_ASSOCIATION_LABELS},
    crypto::conversions::*,
    types::AssociationSetFile,
    io::FileManager,
    error::{CoinUtilsError, Result},
};

/// Manager for handling association set operations
pub struct AssociationManager {
    file_manager: FileManager,
}

impl AssociationManager {
    pub fn new() -> Self {
        Self {
            file_manager: FileManager::new(),
        }
    }
    
    /// Update association set by adding a new label
    pub fn update_association_set(&self, env: &Env, filename: &str, label: &str) -> Result<()> {
        // Try to read existing association set file
        let mut association_set = if self.file_manager.file_exists(filename) {
            self.file_manager.read_association_file(filename)?
        } else {
            // Create new association set file
            AssociationSetFile {
                labels: Vec::new(),
                scope: "default_scope".to_string(),
                root: None,
            }
        };

        // Check if label already exists
        if !association_set.labels.contains(&label.to_string()) {
            // Check if we're at the limit for depth 2 (4 labels max)
            if association_set.labels.len() >= MAX_ASSOCIATION_LABELS {
                return Err(CoinUtilsError::AssociationSetFull);
            }
            
            association_set.labels.push(label.to_string());
            
            // Compute the Merkle tree root for the association set
            if !association_set.labels.is_empty() {
                // Build association set merkle tree (depth 2)
                let mut association_tree = LeanIMT::new(env, ASSOCIATION_TREE_DEPTH);
                
                for label_str in &association_set.labels {
                    let label_fr = decimal_string_to_bls_scalar(env, label_str)
                        .map_err(|e| CoinUtilsError::InvalidDecimal(format!("Invalid association label: {}", e)))?;
                    
                    // Convert BlsScalar to bytes and insert into association tree
                    let label_bytes = lean_imt::bls_scalar_to_bytes(label_fr);
                    association_tree.insert(label_bytes)?;
                }
                
                // Get the root and convert to decimal string
                let association_root_scalar = lean_imt::bytes_to_bls_scalar(&association_tree.get_root());
                association_set.root = Some(bls_scalar_to_decimal_string(&association_root_scalar));
            }
            
            // Save updated association set
            self.file_manager.write_association_file(&association_set, filename)?;
            
            println!("Added label '{}' to association set. Total labels: {}", label, association_set.labels.len());
            if let Some(ref root) = association_set.root {
                println!("Association set root: {}", root);
            }
        } else {
            println!("Label '{}' already exists in association set", label);
        }

        Ok(())
    }
}

impl Default for AssociationManager {
    fn default() -> Self {
        Self::new()
    }
}
