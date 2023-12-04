service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("cache-candles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheCandleNosqlModel {
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
    #[serde(rename = "Expires")]
    pub expires: String,
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
