service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("bidask-statistics")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskStatisticNosqlModel {
    #[serde(rename = "I")]
    pub instrument_id: String,
    #[serde(rename = "P")]
    pub period_type: i32,
    #[serde(rename = "B")]
    pub bid_data: PriceStatisticDataNosqlModel,
    #[serde(rename = "A")]
    pub ask_data: PriceStatisticDataNosqlModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceStatisticDataNosqlModel {
    #[serde(rename = "H")]
    pub high: f64,
    #[serde(rename = "L")]
    pub low: f64,
    #[serde(rename = "O")]
    pub open: f64,
    #[serde(rename = "C")]
    pub close: f64,
    #[serde(rename = "V")]
    pub volume: f64,
    #[serde(rename = "C")]
    pub change_percent: f64,
}

impl BidAskStatisticNosqlModel {
    pub fn generate_pk(period_type: i32) -> String {
        format!("{}", period_type)
    }

    pub fn generate_rk(instrument_id: String) -> String {
        instrument_id
    }
}
