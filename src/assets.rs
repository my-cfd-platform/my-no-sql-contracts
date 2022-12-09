use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const ASSETS_TABLE_NAME: &str = "assets";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Accuracy")]
    pub accuracy: i32,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "IconUrl")]
    pub icon_url: String,
    #[serde(rename = "Type")]
    pub asset_type: i32,
    #[serde(rename = "Weight")]
    pub weight: i32,
    #[serde(rename = "BrandIds")]
    pub brand_ids: Vec<String>,
    #[serde(rename = "CanBeBase")]
    pub can_be_base: bool,
}

impl AssetNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(symbol: String) -> String {
        symbol
    }
}

impl MyNoSqlEntity for AssetNosqlModel {
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