service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("assets")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetNosqlModel {
    #[serde(rename = "Accuracy")]
    pub accuracy: i32,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "IconUrl")]
    pub icon_url: String,
    #[serde(rename = "Type")]
    pub asset_type: i32,
    #[serde(rename = "Weight")]
    pub weight: i32,
    #[serde(rename = "BrandIds")]
    pub brand_ids: Vec<String>,
    #[serde(rename = "CanBeBase")]
    pub can_be_base: bool,
}

impl AssetNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(symbol: String) -> String {
        symbol
    }
}
