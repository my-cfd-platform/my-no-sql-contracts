use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "exchange-assets";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeSettingsNosqlModel {
    pub row_key: String,
    pub partition_key: String,
    pub timestamp: String,
    pub id: String,
    pub quote_refresh_period_secs: i64,
    pub quote_lifetime_secs: i64,
    pub cross_asset_symbol: String,
    pub assets: Vec<ExchangeAssetNosqlModel>,
    pub asset_pairs: Vec<ExchangeAssetPairNosqlModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeAssetNosqlModel {
    pub symbol: String,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub fee_percent: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeAssetPairNosqlModel {
    pub base_asset_symbol: String,
    pub base_min_amount: Option<f64>,
    pub base_max_amount: Option<f64>,
    pub quote_asset_symbol: String,
    pub quote_min_amount: Option<f64>,
    pub quote_max_amount: Option<f64>,
    pub quote_fee_percent: Option<f64>,
}

impl ExchangeSettingsNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk() -> &'static str {
        "*"
    }
}

impl MyNoSqlEntity for ExchangeSettingsNosqlModel {
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
