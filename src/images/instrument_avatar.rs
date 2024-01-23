service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("instrumentsavatar")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentAvatarNosqlModel {
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
