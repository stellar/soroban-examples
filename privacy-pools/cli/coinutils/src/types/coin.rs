use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoinData {
    pub value: String,
    pub nullifier: String,
    pub secret: String,
    pub label: String,
    pub commitment: String,
}

#[derive(Serialize, Deserialize)]
pub struct GeneratedCoin {
    pub coin: CoinData,
    pub commitment_hex: String,
}
