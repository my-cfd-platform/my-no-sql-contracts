use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("bonus-voucher-templates")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoucherTemplateNosqlModel {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Reward")]
    pub reward: VoucherRewardNosqlModel,
    #[serde(rename = "LifetimeMinutes")]
    pub lifetime_minutes: u64,
    #[serde(rename = "Tasks")]
    pub tasks: Vec<VoucherTaskNosqlModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoucherRewardNosqlModel {
    #[serde(rename = "Type")]
    pub r#type: i32,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "Asset")]
    pub asset: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoucherTaskNosqlModel {
    #[serde(rename = "Type")]
    pub r#type: i32,
    #[serde(rename = "Goal")]
    pub goal: f64,
}

impl VoucherTemplateNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}