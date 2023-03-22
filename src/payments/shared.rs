use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoPaymentAssetNosqlModel {
    #[serde(rename = "AssetSymbol")]
    pub asset_symbol: String,
    #[serde(rename = "BlockchainSymbols")]
    pub blockchain_symbols: Vec<String>,
}
