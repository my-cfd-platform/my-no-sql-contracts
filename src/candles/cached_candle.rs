use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const TABLE_NAME: &str = "cached-candles";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedCandleNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "CandleType")]
    pub candle_type: i32,
    #[serde(rename = "Instrument")]
    pub instrument: String,
    #[serde(rename = "BidDataJson")]
    pub bid_data_json: String,
    #[serde(rename = "AskDataJson")]
    pub ask_data_json: String,
    #[serde(rename = "Expires")]
    pub expires: String,
}

impl CachedCandleNosqlModel {
    pub fn generate_pk(instrument: &str) -> &str{
        instrument
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}

impl MyNoSqlEntity for CachedCandleNosqlModel {
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