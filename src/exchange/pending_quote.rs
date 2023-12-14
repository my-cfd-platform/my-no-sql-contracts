service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("exchange-pending-quotes")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangePendingQuoteNosqlModel {
    pub id: String,
    pub trader_id: String,
    pub wallet_id: String,
    pub create_ts_micros: i64,
    pub from_asset_amount: f64,
    pub from_asset_symbol: String,
    pub to_asset_symbol: String,
    pub to_asset_amount: f64,
    pub req_asset_amount: f64,
    pub req_asset_symbol: String,
    pub req_scene: i32,
    pub price: f64,
    pub fee_asset_symbol: String,
    pub fee_asset_amount: f64,
    pub quote_type: i32,
    pub operation_id: String,
}

impl ExchangePendingQuoteNosqlModel {
    pub fn generate_pk(trader_id: &str) -> &str {
        trader_id
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}
