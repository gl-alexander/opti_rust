pub struct MonteCarloParams {
    pub current_asset_price: String,
    pub market_option_price: String,
    pub strike_price: String,
    pub days_to_expire: String,
    pub num_simulations: String,
    pub num_steps: String,
    pub risk_free_rate: String,
    pub implied_vol: String,
}

impl Default for MonteCarloParams {
    fn default() -> Self {
        MonteCarloParams { 
            current_asset_price:    String::from("100.0"), 
            market_option_price:    String::from("5.0"), 
            strike_price:           String::from("105.5"), 
            days_to_expire:         String::from("25"), 
            num_simulations:        String::from("1000"), 
            num_steps:              String::from("10"), 
            risk_free_rate:         String::from("0.05"), 
            implied_vol:            String::from("0.25") 
        }
    }
}