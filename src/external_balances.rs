service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[my_no_sql_entity("external-balances")]
pub struct ExternalBalanceNosqlModel {
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "AssetSymbol")]
    pub asset_symbol: String,
    #[serde(rename = "AssetAmount")]
    pub asset_amount: f64,
    #[serde(rename = "PaymentProvider")]
    pub payment_provider: i32,
    #[serde(rename = "BlockchainSymbol")]
    pub blockchain_symbol: String,
}

impl ExternalBalanceNosqlModel {
    pub fn generate_pk(payment_provider: i32) -> String {
        payment_provider.to_string()
    }

    pub fn generate_rk(asset_symbol: String, blockchain_symbol: String) -> String {
        format!("{asset_symbol}{blockchain_symbol}")
    }
}
