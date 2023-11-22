service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[my_no_sql_entity("payment-provider-assets")]
pub struct PaymentProviderAssetNosqlModel {
    #[serde(rename = "PaymentProvider")]
    pub payment_provider: i32,
    #[serde(rename = "ExternalId")]
    pub external_id: String,
    #[serde(rename = "BlockchainSymbol")]
    pub blockchain_symbol: String,
    #[serde(rename = "AssetSymbol")]
    pub asset_symbol: String,
}

impl PaymentProviderAssetNosqlModel {
    pub fn generate_pk(payment_provider: i32) -> String {
        payment_provider.to_string()
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}
