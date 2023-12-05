service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("quoteprofile")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskNosqlModel {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Bid")]
    pub bid: f64,
    #[serde(rename = "Ask")]
    pub ask: f64,
}

impl BidAskNosqlModel {
    pub fn generate_pk() -> &'static str {
        "qp"
    }

    pub fn generate_rk(symbol: String) -> String {
        symbol
    }
}
