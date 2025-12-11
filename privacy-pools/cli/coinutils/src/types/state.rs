use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StateFile {
    pub commitments: Vec<String>,
    pub scope: String,
    pub association_set: Option<Vec<String>>, // Optional association set labels
}

#[derive(Serialize, Deserialize)]
pub struct AssociationSetFile {
    pub labels: Vec<String>,
    pub scope: String,
    pub root: Option<String>, // Merkle tree root of the association set
}
