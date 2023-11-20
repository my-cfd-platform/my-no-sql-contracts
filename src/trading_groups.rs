service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[my_no_sql_entity("live-tradinggroups")]
pub struct TradingGroupNosqlModel {
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "MarkupProfileId")]
    pub markup_profile_id: String,
    #[serde(rename = "TradingProfileId")]
    pub trading_profile_id: String,
}

impl TradingGroupNosqlModel {
    pub fn generate_pk() -> &'static str {
        "group"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}
