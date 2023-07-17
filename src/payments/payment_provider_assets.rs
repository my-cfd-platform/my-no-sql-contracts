use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "payment-provider-assets";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoBuySettingsNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "PaymentProvider")]
    pub payment_provider: i32,
    #[serde(rename = "ExternalId")]
    pub external_id: String,
    #[serde(rename = "BlockchainSymbol")]
    pub blockchain_symbol: String,
    #[serde(rename = "AssetSymbol")]
    pub asset_symbol: String,
}

impl CryptoBuySettingsNosqlModel {
    pub fn generate_pk(payment_provider: i32) -> String {
        payment_provider.to_string()
    }

    pub fn generate_rk(id: &str) -> &str {
        id
    }
}

impl MyNoSqlEntity for CryptoBuySettingsNosqlModel {
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
