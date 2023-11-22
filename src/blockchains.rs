service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[my_no_sql_entity("blockchains")]
pub struct BlockchainNosqlModel {
    #[serde(rename = "Weight")]
    pub weight: i32,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "EstimatedArrivalTimeSec")]
    pub estimated_arrival_time_sec: i64,
    #[serde(rename = "MinConfirmsCount")]
    pub min_confirms_count: i32,
    #[serde(rename = "BlockUrl")]
    pub block_url: String,
    #[serde(rename = "TxUrl")]
    pub tx_url: String,
    #[serde(rename = "AddressUrl")]
    pub address_url: String,
    #[serde(rename = "SupportsMemo")]
    pub supports_memo: bool,
}

impl BlockchainNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(symbol: &str) -> &str {
        symbol
    }
}
