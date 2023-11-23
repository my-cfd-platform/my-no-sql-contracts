service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("candles-import-tasks")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandlesImportTaskNosqlModel {
    #[serde(rename = "Instrument")]
    pub instrument: String,
    #[serde(rename = "CandleType")]
    pub candle_type: u16,
    #[serde(rename = "StartDatetime")]
    pub start_datetime: String,
    #[serde(rename = "EndDatetime")]
    pub end_datetime: String,
    #[serde(rename = "CurrentDatetime")]
    pub current_datetime: String,
    #[serde(rename = "Source")]
    pub source: i32,
    #[serde(rename = "VolumeMultiplier")]
    pub volume_multiplier: Option<f64>,
}

impl CandlesImportTaskNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(instrument: String, candle_type: u16) -> String {
        format!("{}-{}", instrument, candle_type)
    }
}
