service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[my_no_sql_entity("quoteprofile")]
pub struct BidAskNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
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
