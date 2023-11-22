service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[my_no_sql_entity("payment-methods")]
pub struct PaymentMethodNosqlModel {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "IconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "Weight")]
    pub weight: Option<i32>,
}

impl PaymentMethodNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}
