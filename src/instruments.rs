use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const TABLE_NAME: &str = "instruments";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentNosqlModel {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    //#[serde(rename = "Name")]
    //pub name: Option<String>,
    #[serde(rename = "Digits")]
    pub digits: i32,
    #[serde(rename = "Base")]
    pub base: Option<String>,
    #[serde(rename = "Quote")]
    pub quote: Option<String>,
    #[serde(rename = "TickSize")]
    pub tick_size: f64,
    //#[serde(rename = "SwapScheduleId")]
    //pub swap_schedule_id: String,
    //#[serde(rename = "GroupId")]
    //pub group_id: Option<String>,
    //#[serde(rename = "SubGroupId")]
    //pub sub_grpup_id: Option<String>,
    #[serde(rename = "Weight")]
    pub weight: i32,
    #[serde(rename = "DayTimeout")]
    pub day_timeout: i32,
    #[serde(rename = "NightTimeout")]
    pub night_timeout: i32,
    #[serde(rename = "TradingDisabled")]
    pub trading_disabled: Option<bool>,
}

impl InstrumentNosqlModel {
    pub fn generate_pk() -> &'static str {
        "i"
    }

    pub fn generate_rk(id: String) -> String {
        id
    }
}

impl MyNoSqlEntity for InstrumentNosqlModel {
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