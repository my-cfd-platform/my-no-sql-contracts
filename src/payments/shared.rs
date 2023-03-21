use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentAssetNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "AssetSymbol")]
    pub asset_symbol: String,
    #[serde(rename = "BlockchainSymbols")]
    pub blockchain_symbol: Vec<String>,
}
