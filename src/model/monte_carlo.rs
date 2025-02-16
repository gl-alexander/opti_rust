use crate::model::chart::PriceChart;
use std::error::Error;
use rand_distr::{Normal, Distribution};
use rayon::prelude::*;

use super::{params::MonteCarloParams, utils::days_to_years};

#[derive(Default)]
pub struct MonteCarloPricing {
    pub current_asset_price: f64,
    pub market_option_price: f64,
    pub strike_price: f64,
    pub num_simulations: u16,
    pub num_steps: u16,
    pub risk_free_rate: f64,
    pub implied_vol: f64,
    pub years_to_expire: f64,
}

impl MonteCarloPricing {
    pub fn from_params(params: &MonteCarloParams) -> MonteCarloPricing {
        MonteCarloPricing {
            current_asset_price: params.current_asset_price.parse::<f64>().unwrap_or(100.0),
            market_option_price: params.market_option_price.parse::<f64>().unwrap_or(0.0),
            strike_price: params.strike_price.parse::<f64>().unwrap_or(0.0),
            num_simulations: params.num_simulations.parse::<u16>().unwrap_or(1000),
            num_steps: params.num_steps.parse::<u16>().unwrap_or(5),
            risk_free_rate: params.risk_free_rate.parse::<f64>().unwrap_or(0.0),
            implied_vol: params.implied_vol.parse::<f64>().unwrap_or(0.03),
            years_to_expire: days_to_years(params.days_to_expire.parse::<u16>().unwrap_or(30)),
        }
    }

    pub fn price(&self, price_chart: &PriceChart) -> Result<f64, Box<dyn Error>> {
        let dt = self.years_to_expire / self.num_steps as f64;

        let sum_payoff: f64 = (0..self.num_simulations)
            .into_par_iter() // Run simulations in parallel
            .map(|_| {
                let mut st = price_chart.underlying_price();
                for _ in 0..self.num_steps {
                    let w = wiener_increment(dt);
                    st *= 1.0 + self.risk_free_rate * dt + self.implied_vol * w;
                }
                (st - self.strike_price).max(0.0) 
            })
            .sum(); 

        let average_payoff = (sum_payoff / self.num_simulations as f64)
            * (-self.risk_free_rate * self.years_to_expire).exp();
        Ok(average_payoff)

    }
}



pub(crate)fn wiener_increment(dt: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::rng();

    let sample = normal.sample(&mut rng);

    sample * dt.sqrt()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::chart::PriceChart;

    #[test]
    fn test_from_params() {
        let params = MonteCarloParams {
            current_asset_price: "100.0".to_string(),
            market_option_price: "10.0".to_string(),
            strike_price: "100.0".to_string(),
            num_simulations: "1000".to_string(),
            days_to_expire: "30".to_string(),
            risk_free_rate: "0.05".to_string(),
            implied_vol: "0.2".to_string(),
            num_steps: "10".to_string(),
        };

        let mc = MonteCarloPricing::from_params(&params);
        assert_eq!(mc.current_asset_price, 100.0);
        assert_eq!(mc.market_option_price, 10.0);
        assert_eq!(mc.strike_price, 100.0);
        assert_eq!(mc.num_simulations, 1000);
        assert_eq!(mc.num_steps, 10);
    }

    #[test]
    fn test_price() {
        let mc = MonteCarloPricing {
            current_asset_price: 100.0,
            market_option_price: 10.0,
            strike_price: 100.0,
            num_simulations: 1000,
            num_steps: 10,
            risk_free_rate: 0.05,
            implied_vol: 0.2,
            years_to_expire: 1.0,
        };

        let price_chart = PriceChart::default();
        let price_result = mc.price(&price_chart);
        
        assert!(price_result.is_ok());
        assert!(price_result.unwrap() > 0.0);
    }

    #[test]
    fn test_wiener_increment() {
        let dt = 0.01;
        let w = wiener_increment(dt);
        assert!(w.is_finite());
    }
}
