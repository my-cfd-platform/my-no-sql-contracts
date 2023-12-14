service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("exchange-assets")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeAssetNosqlModel {
    pub symbol: String,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub fee_percent: Option<f64>,
}

impl ExchangeAssetNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(symbol: &str) -> &str {
        symbol
    }
}
