service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("crypto-buy-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoBuySettingsNosqlModel {
    #[serde(rename = "Priority")]
    pub priority: i32,
    #[serde(rename = "PaymentProvider")]
    pub payment_provider: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Assets")]
    pub assets: Vec<CryptoBuyAssetNosqlModel>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "ConvertEnabled")]
    pub convert_enabled: bool,
}

impl CryptoBuySettingsNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoBuyAssetNosqlModel {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "FiatAssetSymbol")]
    pub fiat_asset_symbol: String,
    #[serde(rename = "CryptoAssetSymbols")]
    pub crypto_asset_symbols: Vec<String>,
    #[serde(rename = "MinAmount")]
    pub min_amount: Option<f64>,
    #[serde(rename = "MaxAmount")]
    pub max_amount: Option<f64>,
}