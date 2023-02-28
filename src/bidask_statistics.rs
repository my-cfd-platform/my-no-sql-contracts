use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "bidask-statistics";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskStatisticNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "i")]
    pub instrument_id: String,
    #[serde(rename = "p")]
    pub period_type: i32,
    #[serde(rename = "b")]
    pub bid_data: PriceStatisticDataNosqlModel,
    #[serde(rename = "a")]
    pub ask_data: PriceStatisticDataNosqlModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceStatisticDataNosqlModel {
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "v")]
    pub volume: f64,
}

impl BidAskStatisticNosqlModel {
    pub fn generate_pk(period_type: i32) -> String {
        format!("{}", period_type)
    }

    pub fn generate_rk(instrument_id: String) -> String {
        instrument_id
    }
}

impl MyNoSqlEntity for BidAskStatisticNosqlModel {
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