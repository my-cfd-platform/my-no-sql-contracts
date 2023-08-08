use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "exchange-assets";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeAssetNosqlModel {
    pub row_key: String,
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    pub symbol: String,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub fee_percent: Option<f64>,
}

impl ExchangeAssetNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(symbol: &str) -> &str {
        symbol
    }
}

impl MyNoSqlEntity for ExchangeAssetNosqlModel {
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
