use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "blockchains";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockchainNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Weight")]
    pub weight: i32,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "EstimatedArrivalTimeSec")]
    pub estimated_arrival_time_sec: i64,
    #[serde(rename = "MinConfirmsCount")]
    pub min_confirms_count: i32,
    #[serde(rename = "BlockUrl")]
    pub block_url: String,
    #[serde(rename = "TxUrl")]
    pub tx_url: String,
    #[serde(rename = "AddressUrl")]
    pub address_url: String,
}

impl BlockchainNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(symbol: &str) -> &str {
        symbol
    }
}

impl MyNoSqlEntity for BlockchainNosqlModel {
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