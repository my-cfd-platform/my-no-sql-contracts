use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};
use crate::payments::shared::PaymentAssetNosqlModel;

pub const TABLE_NAME: &str = "deposit-methods";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DepositMethodNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "PaymentType")]
    pub payment_type: i32,
    #[serde(rename = "PaymentProvider")]
    pub payment_provider: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Assets")]
    pub assets: Vec<PaymentAssetNosqlModel>,
}

impl DepositMethodNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}

impl MyNoSqlEntity for DepositMethodNosqlModel {
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