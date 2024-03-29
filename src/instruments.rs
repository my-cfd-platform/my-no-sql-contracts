service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("instruments")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentNosqlModel {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Digits")]
    pub digits: i32,
    #[serde(rename = "Base")]
    pub base_asset: String,
    #[serde(rename = "Quote")]
    pub quote_asset: String,
    //#[serde(rename = "TickSize")]
    //pub tick_size: f64,
    //#[serde(rename = "SwapScheduleId")]
    //pub swap_schedule_id: String,
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,
    #[serde(rename = "Weight")]
    pub weight: i32,
    #[serde(rename = "DayTimeout")]
    pub day_timeout: i32,
    #[serde(rename = "NightTimeout")]
    pub night_timeout: i32,
    #[serde(rename = "TradingDisabled")]
    pub trading_disabled: Option<bool>,
}

impl InstrumentNosqlModel {
    pub fn generate_pk() -> &'static str {
        "i"
    }

    pub fn generate_rk(id: String) -> String {
        id
    }
}
