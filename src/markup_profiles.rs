service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[my_no_sql_entity("markup-profiles")]
pub struct MarkupProfileNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Instruments")]
    pub instruments: Vec<MarkupProfileInstrumentNosqlModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkupProfileInstrumentNosqlModel {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Bid")]
    pub ask: i32,
    #[serde(rename = "Ask")]
    pub bid: i32,
}

impl MarkupProfileNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}
