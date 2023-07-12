use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "crypto-buy-settings";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoBuySettingsNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
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
}

impl CryptoBuySettingsNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}

impl MyNoSqlEntity for CryptoBuySettingsNosqlModel {
    const TABLE_NAME: &'static str = TABLE_NAME;

    fn get_partition_key(&self) -> &str {
        &self.partition_key
    }

    fn get_row_key(&self) -> &str {
        &self.row_key
    }

    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::parse_iso_string(self.timestamp.as_str())
            .expect("Failed to parse timestamp")
            .unix_microseconds
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