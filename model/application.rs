use crate::gui::chart::ChartDisplayState;

use crate::model::chart::PriceChart;
use reqwest::Error;
use crate::model::request::{StockData};

pub struct OptiRust {
    pub chart: PriceChart,
    pub show_import: bool,
    pub imported_index: String,
    pub api_key: String,
    pub require_api_key: bool,
    pub error_message: Option<String>
}

impl Default for OptiRust {
    fn default() -> OptiRust {
        OptiRust{
            chart: PriceChart::default(), 
            show_import: false, 
            imported_index: String::new(), 
            api_key: String::from(""), 
            require_api_key: false,
            error_message: None,
        }
    }
}

impl OptiRust {
    pub async fn get_index_data(& mut self) {
        if (self.api_key.is_empty()) {
            self.error_message = Some(String::from("No Alpha Vantage API key was specified"));
            return;
        }
        if (self.imported_index.is_empty()) {
            self.error_message = Some(String::from("No Stock index was specified"));
        }
        let symbol = "GOOGL";
        let api_key = std::env::var("API_KEY").expect("API_KEY environment variable not found");
        let daily_prices_url = format!(
            "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}&outputsize=30",
            self.imported_index, self.api_key
        );
        let daily_prices_response = reqwest::blocking::get(&daily_prices_url);
        if daily_prices_response.is_err() {
            self.error_message = Some(String::from(format!("Error while making request. {}", daily_prices_response.err().unwrap())));
            return;
        }
        let parsed_response = daily_prices_response.unwrap().json::<StockData>();
        if parsed_response.is_err() {
            self.error_message = Some(String::from(format!("Error while parsing response. {}", parsed_response.err().unwrap())));
            return;
        }
        self.chart = PriceChart::from_json(parsed_response.unwrap());
    }
}