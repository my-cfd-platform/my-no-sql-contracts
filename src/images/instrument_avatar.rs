use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const TABLE_NAME: &str = "instrumentsavatar";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentAvatarNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
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

impl MyNoSqlEntity for InstrumentAvatarNosqlModel {
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
