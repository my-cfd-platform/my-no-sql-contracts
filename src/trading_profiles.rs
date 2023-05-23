use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "live-tradingprofiles";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingProfileNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "MarginCallPercent")]
    pub margin_call_percent: String,
    #[serde(rename = "StopOutPercent")]
    pub stop_out_percent: String,
    #[serde(rename = "TopUpPercent")]
    pub top_up_percent: String,
    #[serde(rename = "Instruments")]
    pub instruments: Vec<TradingProfileInstrumentNosqlModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingProfileInstrumentNosqlModel {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "MinInvestAmount")]
    pub min_invest_amount: Option<f64>,
    #[serde(rename = "MaxCumulativeInvestVolume")]
    pub max_cumulative_invest_volume: Option<f64>,
}

impl TradingProfileNosqlModel {
    pub fn generate_pk() -> &'static str {
        "profile"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}

impl MyNoSqlEntity for TradingProfileNosqlModel {
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