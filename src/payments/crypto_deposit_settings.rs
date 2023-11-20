service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};
use crate::payments::shared::CryptoPaymentAssetNosqlModel;

#[my_no_sql_entity("crypto-deposit-settings")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoDepositSettingsNosqlModel {
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Priority")]
    pub priority: i32,
    #[serde(rename = "PaymentProvider")]
    pub payment_provider: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Assets")]
    pub assets: Vec<CryptoPaymentAssetNosqlModel>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl CryptoDepositSettingsNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}
