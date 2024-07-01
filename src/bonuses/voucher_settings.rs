use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("bonus-voucher-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoucherSettings {
    pub name: String,
    pub asset_amount: f64,
    pub asset_symbol: String,
    pub lifetime_minutes: u64,
    pub tasks: Vec<VoucherTask>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoucherTask {
    pub task_type: i32,
    pub task_amount: f64,
}

impl VoucherSettings {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}