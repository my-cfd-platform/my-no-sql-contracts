service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[my_no_sql_entity("instrumentsavatar")]
pub struct InstrumentAvatarNosqlModel {
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Avatar")]
    pub avatar: String,
}

impl InstrumentAvatarNosqlModel {
    pub fn generate_pk(id: String) -> String {
        id
    }

    pub fn generate_rk(img_type: String) -> String {
        img_type
    }
}
