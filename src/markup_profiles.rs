service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("markup-profiles")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkupProfileNosqlModel {
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
