use crate::model::application::OptiRust;
use crate::model::monte_carlo::MonteCarloPricing;

#[derive(Debug, Clone)]
pub enum Message {
    ShowImport,
    HideImport,
    ImportData,
    ImportIndexChanged(String),
    SubmitApiKey,
    HideSubmitApiKey,
    ApiKeyChanged(String),
    ClearError,

    AssetPriceChanged(String),
    StrikePriceChanged(String),
    MarketPriceChanged(String),
    ImpliedVolChanged(String),
    RiskFreeRateChanged(String),
    DaysToExpireChanged(String),
    NumStepsChanged(String),
    NumSimulationsChanged(String),
    UpdateParameters,
    RunMonteCarlo,
}

impl OptiRust {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ShowImport => {
                if self.api_key.is_empty() {
                    self.require_api_key = true;

                }
                else {
                    self.show_import = true
                }
            },
            Message::HideImport => self.show_import = false, 
            Message::ImportData => {
                self.get_index_data();
                self.show_import = false;
            },
            Message::ImportIndexChanged(value) => self.index_value_text = value,
            Message::ApiKeyChanged(value) => self.api_key = value,
            Message::SubmitApiKey => {
                self.require_api_key = false;
                self.show_import = true;
            },
            Message::HideSubmitApiKey => {
                self.require_api_key = false;
                self.api_key = String::new();
            },
            Message::ClearError => self.error_message = None,
            Message::AssetPriceChanged(value) => self.monte_carlo_params.current_asset_price = value,
            Message::StrikePriceChanged(value) => self.monte_carlo_params.strike_price = value,
            Message::MarketPriceChanged(value) => self.monte_carlo_params.market_option_price = value,
            Message::ImpliedVolChanged(value) => self.monte_carlo_params.implied_vol = value,
            Message::RiskFreeRateChanged(value) => self.monte_carlo_params.risk_free_rate = value,
            Message::DaysToExpireChanged(value) => self.monte_carlo_params.days_to_expire = value,
            Message::NumSimulationsChanged(value) => self.monte_carlo_params.num_simulations = value,
            Message::NumStepsChanged(value) => self.monte_carlo_params.num_steps = value,
            Message::UpdateParameters => {
                self.monte_carlo_pricing = MonteCarloPricing::from_params(&self.monte_carlo_params);
                self.monte_carlo_params.implied_vol = self.monte_carlo_pricing.implied_volatility().to_string();
            }
            Message::RunMonteCarlo => {
                self.monte_carlo_pricing = MonteCarloPricing::from_params(&self.monte_carlo_params);
                let output = self.monte_carlo_pricing.price(&self.chart);
                if output.is_ok() {
                    self.pricing_result = output.ok();
                } else {
                    self.error_message = Some(output.err().unwrap().to_string());
                }
            }
        }
    }
}