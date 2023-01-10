use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const ASSETS_TABLE_NAME: &str = "candles-import-tasks";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandlesImportTaskNoSqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Instrument")]
    pub instrument: String,
    #[serde(rename = "CandleType")]
    pub candle_type: u16,
    #[serde(rename = "StartDatetime")]
    pub start_datetime: String,
    #[serde(rename = "EndDatetime")]
    pub end_datetime: String,
    #[serde(rename = "LastImportDatetime")]
    pub last_import_datetime: Option<String>,
}

impl CandlesImportTaskNoSqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(instrument: String, candle_type: u16) -> String {
        format!("{}-{}", instrument, candle_type)
    }
}

impl MyNoSqlEntity for CandlesImportTaskNoSqlModel {
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