service_sdk::macros::use_my_no_sql_entity!();
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

#[my_no_sql_entity("cached-positions")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PositionNosqlModel {
    pub expires: String,

    pub id: String,
    pub last_update_date: DateTimeAsMicroseconds,
    pub open_date: DateTimeAsMicroseconds,
    pub asset_pnls: HashMap<String, f64>,
    pub open_asset_prices: HashMap<String, f64>,
    pub activate_date: Option<DateTimeAsMicroseconds>,
    pub activate_price: Option<f64>,
    pub activate_asset_prices: HashMap<String, f64>,
    pub close_date: Option<DateTimeAsMicroseconds>,
    pub close_price: Option<f64>,
    pub close_asset_prices: HashMap<String, f64>,
    pub pnl: Option<f64>,
    pub order: OrderNosqlModel,
    pub status: i32,
    pub close_reason: Option<i32>,
    pub top_ups: Vec<TopUpNosqlModel>,
    pub open_price: f64,
    pub total_invest_assets: HashMap<String, f64>,
    pub invest_bonus_assets: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OrderNosqlModel {
    pub id: String,
    pub trader_id: String,
    pub wallet_id: String,
    pub instrument: String,
    pub base_asset: String,
    pub invest_assets: HashMap<String, f64>,
    pub leverage: f64,
    pub create_date: i64,
    pub side: i32,
    pub stop_out_percent: f64,
    pub margin_call_percent: f64,
    pub top_up_enabled: bool,
    pub top_up_percent: f64,
    pub desire_price: Option<f64>,
    pub order_type: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TopUpNosqlModel {
    pub id: String,
    pub date: i64,
    pub asset_amounts: HashMap<String, f64>,
    pub instrument_price: f64,
    pub asset_prices: HashMap<String, f64>,
    pub bonus_amounts: HashMap<String, f64>
}