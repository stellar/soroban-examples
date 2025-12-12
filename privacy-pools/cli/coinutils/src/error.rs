use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoinUtilsError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Hex conversion error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("Invalid decimal string: {0}")]
    InvalidDecimal(String),

    #[error("Commitment not found in state")]
    CommitmentNotFound,

    #[error("Label not found in association set")]
    LabelNotFound,

    #[error("Association set is full")]
    AssociationSetFull,

    #[error("Merkle proof generation failed")]
    ProofGenerationFailed,

    #[error("Invalid byte length: expected 32, got {0}")]
    InvalidByteLength(usize),

    #[error("Invalid decimal character: {0}")]
    InvalidDecimalCharacter(char),

    #[error("LeanIMT error: {0}")]
    LeanIMT(String),
}

impl From<&str> for CoinUtilsError {
    fn from(err: &str) -> Self {
        CoinUtilsError::LeanIMT(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, CoinUtilsError>;
