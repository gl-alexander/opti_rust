use statrs::distribution::{Continuous, ContinuousCDF, Normal};
use crate::model::monte_carlo::MonteCarloPricing;

const DAYS_IN_YEAR: f64 = 365.0;

const SIGMA_INITIAL_GUESS: f64 = 2.0;
const TOLERANCE: f64 = 0.0001;

impl MonteCarloPricing {
    pub fn implied_volatility(&self) -> f64 {
        let mut low: f64 = 0.01;
        let mut high: f64 = SIGMA_INITIAL_GUESS; 
        let mut mid: f64;
    
        while (high - low).abs() > TOLERANCE {
            mid = (low + high) / 2.0;
            let price = self.black_scholes_call_price(mid);
    
            if price > self.market_option_price {
                high = mid;
            } else {
                low = mid;
            }
        }
    
        (low + high) / 2.0 // Return estimated implied volatility
    
    }


    pub fn black_scholes_call_price(&self, sigma: f64) -> f64 {
        let d1 = ((self.strike_price / self.strike_price).ln() + (self.risk_free_rate + 0.5 * sigma.powi(2)) * self.years_to_expire) / (sigma * self.years_to_expire.sqrt());
        let d2 = d1 - sigma * self.years_to_expire.sqrt();
        let normal = Normal::standard();

        let n_d1 = normal.cdf(d1); // CDF of standard normal distribution
        let n_d2 = normal.cdf(d2);

        self.current_asset_price * n_d1 - self.strike_price * (-self.risk_free_rate * self.years_to_expire).exp() * n_d2
    }

    pub fn black_scholes_vega(&self, sigma: f64) -> f64 {
        let d1 = (self.current_asset_price.ln() / self.strike_price.ln() + (self.risk_free_rate + 0.5 * sigma.powi(2)) * self.years_to_expire) 
            / (sigma * self.years_to_expire.sqrt());
        let normal = Normal::standard();
        let n_prime_d1 = normal.pdf(d1); // Standard normal PDF for d1

        self.current_asset_price * n_prime_d1 * self.years_to_expire.sqrt()
    }
}

pub fn days_to_years(days: u16) -> f64 {
    days as f64 / DAYS_IN_YEAR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_scholes_call_price() {
        let mc = MonteCarloPricing {
            current_asset_price: 100.0,
            strike_price: 100.0,
            risk_free_rate: 0.05,
            years_to_expire: 1.0,
            market_option_price: 10.0,
            num_simulations: 1000,
            num_steps: 10,
            implied_vol: 0.25,
        };
        
        let sigma = 0.2;
        let price = mc.black_scholes_call_price(sigma);
        
        assert!(price > 0.0);
    }
    
    #[test]
    fn test_implied_volatility() {
        let mc = MonteCarloPricing {
            current_asset_price: 100.0,
            strike_price: 100.0,
            risk_free_rate: 0.05,
            years_to_expire: 1.0,
            market_option_price: 10.0,
            num_simulations: 1000,
            num_steps: 10,
            implied_vol: 0.25,
        };
        
        let iv = mc.implied_volatility();
        
        assert!(iv > 0.0);
    }

    #[test]
    fn test_black_scholes_vega() {
        let mc = MonteCarloPricing {
            current_asset_price: 100.0,
            strike_price: 100.0,
            risk_free_rate: 0.05,
            years_to_expire: 1.0,
            market_option_price: 10.0,
            num_simulations: 1000,
            num_steps: 10,
            implied_vol: 0.25,
        };
        
        let sigma = 0.2;
        let vega = mc.black_scholes_vega(sigma);
        
        assert!(vega > 0.0);
    }
    
    #[test]
    fn test_days_to_years() {
        assert_eq!(days_to_years(365), 1.0);
        assert_eq!(days_to_years(730), 2.0);
        assert_eq!(days_to_years(0), 0.0);
    }
}
