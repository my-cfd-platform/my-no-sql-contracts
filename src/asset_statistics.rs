service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("asset-statistics")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetStatisticNosqlModel {
    #[serde(rename = "TimeStamp")]
    pub timestamp: String,
    #[serde(rename = "Roi")]
    pub roi: f64,
    #[serde(rename = "MaxSupply")]
    pub max_supply: f64,
    #[serde(rename = "CirculatingSupply")]
    pub circulating_supply: f64,
    #[serde(rename = "MarketCap")]
    pub market_cap: f64,
    #[serde(rename = "FullyDilutedValuation")]
    pub fully_diluted_valuation: f64,
    #[serde(rename = "Rank")]
    pub rank: i32,
    #[serde(rename = "LastPrice")]
    pub last_price: f64,
    #[serde(rename = "Symbol")]
    pub symbol: String,
}

impl AssetStatisticNosqlModel {
    pub fn generate_pk() -> &'static str {
        "*"
    }

    pub fn generate_rk(symbol: String) -> String {
        symbol
    }
}
