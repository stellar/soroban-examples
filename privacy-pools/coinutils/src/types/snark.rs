use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SnarkInput {
    #[serde(rename = "withdrawnValue")]
    pub withdrawn_value: String,
    pub label: String,
    pub value: String,
    pub nullifier: String,
    pub secret: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    #[serde(rename = "stateIndex")]
    pub state_index: String,
    #[serde(rename = "stateSiblings")]
    pub state_siblings: Vec<String>,
    #[serde(rename = "associationRoot")]
    pub association_root: String,
    #[serde(rename = "labelIndex")]
    pub label_index: String,
    #[serde(rename = "labelSiblings")]
    pub label_siblings: Vec<String>,
}
