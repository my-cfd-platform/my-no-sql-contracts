use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoPaymentAssetNosqlModel {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "AssetSymbol")]
    pub asset_symbol: String,
    #[serde(rename = "BlockchainSymbols")]
    pub blockchain_symbols: Vec<String>,
    #[serde(rename = "MinAmount")]
    pub min_amount: Option<f64>,
    #[serde(rename = "MaxAmount")]
    pub max_amount: Option<f64>,
    #[serde(rename = "FeeAmount")]
    pub fee_amount: Option<f64>,
    #[serde(rename = "FeeType")]
    pub fee_type: FeeTypeNosql,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FeeTypeNosql {
    None = 0,
    Absolute = 1,
    Percent = 2,
}