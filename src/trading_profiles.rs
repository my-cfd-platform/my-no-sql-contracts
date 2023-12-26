service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("live-tradingprofiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingProfileNosqlModel {
    #[serde(rename = "MarginCallPercent")]
    pub margin_call_percent: f64,
    #[serde(rename = "StopOutPercent")]
    pub stop_out_percent: f64,
    #[serde(rename = "TopUpPercent")]
    pub top_up_percent: f64,
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
    #[serde(rename = "Leverages")]
    pub leverages: Vec<i32>,
    #[serde(rename = "OpenPositionMaxDelayMs")]
    pub max_open_position_delay_millis: i32,
    #[serde(rename = "OpenPositionMinDelayMs")]
    pub min_open_position_delay_millis: i32,
}

impl TradingProfileNosqlModel {
    pub fn generate_pk() -> &'static str {
        "profile"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}
