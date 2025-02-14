use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct StockData {
    #[serde(rename = "Time Series (Daily)")]
    pub time_series: HashMap<String, DailyPrice>,  // Date as key, price data as value
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DailyPrice {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}