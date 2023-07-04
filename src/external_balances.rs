use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const TABLE_NAME: &str = "external-balances";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalBalanceNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
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

impl MyNoSqlEntity for ExternalBalanceNosqlModel {
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
