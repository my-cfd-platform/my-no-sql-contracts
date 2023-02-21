use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "bidasks-history";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskHistoryNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "InstumentId")]
    pub instrument_id: String,
    #[serde(rename = "PeriodType")]
    pub period_type: i32,
    #[serde(rename = "Bid")]
    pub bid: PriceHistoryNosqlModel,
    #[serde(rename = "Ask")]
    pub ask: PriceHistoryNosqlModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceHistoryNosqlModel {
    #[serde(rename = "High")]
    pub high: f64,
    #[serde(rename = "Low")]
    pub low: f64,
    #[serde(rename = "ChangeValue")]
    pub change_value: f64,
    #[serde(rename = "ChangePercent")]
    pub change_percent: f64,
    #[serde(rename = "BaseVolume")]
    pub base_volume: f64,
}

impl BidAskHistoryNosqlModel {
    pub fn generate_pk(period_type: i32) -> String {
        format!("{}", period_type)
    }

    pub fn generate_rk(instrument_id: String) -> String {
        instrument_id
    }
}

impl MyNoSqlEntity for BidAskHistoryNosqlModel {
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