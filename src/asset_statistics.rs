use serde::*;
use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub const ASSET_STATISTICS_TABLE_NAME: &str = "asset-statistics";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetStatisticNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Roi")]
    pub roi: f64,
    #[serde(rename = "MaxSupply")]
    pub max_supply: f64,
    #[serde(rename = "CirculatingSupply")]
    pub circulating_supply: f64,
    #[serde(rename = "MarketCap")]
    pub market_cap: f64,
    #[serde(rename = "Rank")]
    pub rank: i32,
    #[serde(rename = "LastPrice")]
    pub last_price: f64,
    #[serde(rename = "Symbol")]
    pub symbol: String,
}

impl AssetStatisticNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(symbol: String) -> String {
        symbol
    }
}

impl MyNoSqlEntity for AssetStatisticNosqlModel {
    const TABLE_NAME: &'static str = ASSET_STATISTICS_TABLE_NAME;

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