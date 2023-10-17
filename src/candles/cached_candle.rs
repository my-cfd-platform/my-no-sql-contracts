use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const TABLE_NAME: &str = "cache-candles";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheCandleNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Expires")]
    pub expires: String,

    #[serde(rename = "T")]
    pub candle_type: i32,
    #[serde(rename = "I")]
    pub instrument: String,
    #[serde(rename = "B")]
    pub bid_data: CacheCandleDataNosqlModel,
    #[serde(rename = "A")]
    pub ask_data: CacheCandleDataNosqlModel,
    #[serde(rename = "D")]
    pub date_micros: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheCandleDataNosqlModel {
    #[serde(rename = "O")]
    pub open: String,
    #[serde(rename = "C")]
    pub close: String,
    #[serde(rename = "H")]
    pub high: String,
    #[serde(rename = "L")]
    pub low: String,
    #[serde(rename = "V")]
    pub volume: String,
}

impl CacheCandleNosqlModel {
    pub fn generate_pk(instrument: &str) -> &str{
        instrument
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}

impl MyNoSqlEntity for CacheCandleNosqlModel {
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