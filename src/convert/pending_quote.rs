use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "pending-quotes";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PendingQuoteNosqlModel {
    pub row_key: String,
    pub partition_key: String,
    pub timestamp: String,
    pub expires: String,
    pub id: String,
    pub trader_id: String,
    pub wallet_id: String,
    pub create_ts_micros: i64,
    pub expire_ts_micros: i64,
    pub from_asset_amount: f64,
    pub from_asset_symbol: String,
    pub to_asset_symbol: String,
    pub to_asset_amount: f64,
    pub rfq_asset_amount: f64,
    pub rfq_asset_symbol: String,
    pub price: f64,
    pub fee_asset_symbol: String,
    pub fee_asset_amount: f64,
    pub quote_type: i32,
}

impl PendingQuoteNosqlModel {
    pub fn generate_pk(trader_id: &str) -> &str {
        trader_id
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}

impl MyNoSqlEntity for PendingQuoteNosqlModel {
    const TABLE_NAME: &'static str = TABLE_NAME;

    fn get_partition_key(&self) -> &str {
        &self.partition_key
    }

    fn get_row_key(&self) -> &str {
        &self.row_key
    }

    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::parse_iso_string(self.timestamp.as_str())
            .expect("Failed to parse timestamp")
            .unix_microseconds
    }
}