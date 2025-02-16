use crate::model::chart::PriceChart;
use crate::model::request::StockData;

use super::{monte_carlo::MonteCarloPricing, params::MonteCarloParams};

pub struct OptiRust {
    pub chart: PriceChart,
    pub show_import: bool,
    pub imported_index: String,
    pub index_value_text: String,
    pub api_key: String,
    pub require_api_key: bool,
    pub error_message: Option<String>,
    pub monte_carlo_pricing: MonteCarloPricing,
    pub monte_carlo_params: MonteCarloParams,
    pub pricing_result: Option<f64>,
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
            monte_carlo_pricing: MonteCarloPricing::default(),
            monte_carlo_params: MonteCarloParams::default(),
            index_value_text: String::from(""),
            pricing_result: None,
        }
    }
}

impl OptiRust {
    pub fn get_index_data(& mut self) {
        if self.api_key.is_empty() {
            self.error_message = Some(String::from("No Alpha Vantage API key was specified"));
            return;
        }
        if self.index_value_text.is_empty() {
            self.error_message = Some(String::from("No Stock index was specified"));
            return;
        }
        
        let daily_prices_url = format!(
            "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}&outputsize=30",
            self.index_value_text, self.api_key
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
        self.imported_index = self.index_value_text.clone();
        self.chart = PriceChart::from_json(parsed_response.unwrap());
        let stock_price = self.chart.data.iter().map(|d| &d.price).last().unwrap();
        self.monte_carlo_params.current_asset_price = stock_price.to_string();
        self.monte_carlo_params.strike_price = (stock_price * 1.05).to_string();
    }
}