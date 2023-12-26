service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("live-tradinggroups")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingGroupNosqlModel {
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
