use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("crypto-buy-prices")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoBuyPricesNosqlModel {
    #[serde(rename = "FiatByCrypto")]
    pub fiat_by_crypto: HashMap<String, CryptoBuyFiatPricesNosqlModel>,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoBuyFiatPricesNosqlModel {
    #[serde(rename = "PricesByFiat")]
    pub prices_by_fiat: HashMap<String, f64>,
}

impl CryptoBuyPricesNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}