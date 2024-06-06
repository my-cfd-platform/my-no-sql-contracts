use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("crypto-sell-prices")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoSellPricesNosqlModel {
    #[serde(rename = "Crypto")]
    pub crypto: String,
    #[serde(rename = "PricesByFiat")]
    pub prices_by_fiat: HashMap<String, f64>,
    #[serde(rename = "Expires")]
    pub expires: String,
}

impl CryptoSellPricesNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}