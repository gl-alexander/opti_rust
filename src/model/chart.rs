use chrono::{Days, NaiveDate};
use core::f64;

use super::request::StockData;

pub struct PriceChart {
    pub data: Vec<DataPoint>,
    pub squared_sum: f64,
    pub sum: f64,
    pub max_price: f64,
    pub min_price: f64,
    pub refresh_chart: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct DataPoint {
    pub price: f64,
    pub date: NaiveDate,
}

impl DataPoint {
    pub fn new(price: f64, date: NaiveDate) -> DataPoint {
        DataPoint{price, date}
    }
}

impl PriceChart {
    pub fn new<'a>(input_data: &'a Vec<DataPoint>) -> PriceChart {
        let pc_sum = input_data.iter().map(|&d| (d.price)).sum();
        let squared_sum = input_data.iter().map(|&d| (d.price).powi(2)).sum();
        PriceChart{
            data: input_data.to_vec(), 
            squared_sum: squared_sum, 
            sum: pc_sum, 
            max_price: input_data.iter().map(|&p| p.price).fold(f64::MIN, |a, b| a.max(b)),
            min_price: input_data.iter().map(|&p| p.price).fold(f64::INFINITY, |a, b| a.min(b)),
            refresh_chart: false,
        }
    }

    pub fn from_prices_and_date(input_data: Vec<f64>, starting_date: NaiveDate) -> PriceChart {
        let data_points: Vec<DataPoint> = input_data
        .iter()
        .enumerate() // Get index and value
        .map(|(i, &price)| {
            let date = starting_date.checked_add_days(Days::new(i as u64)).unwrap();
            DataPoint::new(price, date)
        })
        .collect();
        PriceChart::new(&data_points)
    }

    pub fn from_json(raw_data: StockData) -> PriceChart {
        // Convert to Vec<DataPoint>
        let mut data_points: Vec<DataPoint> = raw_data.time_series
            .into_iter()
            .map(|(date_str, daily_price)| {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
                let price = daily_price.close.parse::<f64>().unwrap();
                DataPoint { date, price }
            })
            .collect();

        // Sort by date (oldest first)
        data_points.sort_by_key(|dp| dp.date);

        PriceChart::new(&data_points)
    }
 
    pub fn change_price(& mut self, index: usize, new_price: f64) {
        let current = self.data[index].price;
        self.sum += new_price - current;
        self.squared_sum += new_price.powi(2) - current.powi(2);
        self.data[index].price = new_price;

        for dp in &self.data {
            self.min_price = self.min_price.min(dp.price);
            self.max_price = self.max_price.max(dp.price);
        }
    }

    pub fn variance(&self) -> f64 {
        self.squared_sum / (self.data.len() as f64) - (self.sum / (self.data.len() as f64)).powi(2)
    }

    pub fn underlying_price(&self) -> f64 {
        if let Some(dp) = self.data.last() {
            dp.price
        } else {
            0.0f64
        }
    }
}

impl Default for PriceChart {
    fn default() -> Self {
        PriceChart::from_prices_and_date(
            vec!(113.0, 116.5, 137.0, 145.3, 101.32, 104.33, 120.32, 100.4), 
            NaiveDate::from_ymd_opt(2025, 1, 3).unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_data_point_creation() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let dp = DataPoint::new(100.0, date);
        assert_abs_diff_eq!(dp.price, 100.0, epsilon = 1e-6);
        assert_eq!(dp.date, date);
    }

    #[test]
    fn test_price_chart_creation() {
        let data = vec![
            DataPoint::new(100.0, NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            DataPoint::new(105.0, NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()),
        ];
        let chart = PriceChart::new(&data);
        assert_abs_diff_eq!(chart.sum, 205.0, epsilon = 1e-6);
        assert_abs_diff_eq!(chart.max_price, 105.0, epsilon = 1e-6);
        assert_abs_diff_eq!(chart.min_price, 100.0, epsilon = 1e-6);
    }

    #[test]
    fn test_change_price() {
        let mut chart = PriceChart::default();
        let old_price = chart.data[0].price;
        chart.change_price(0, 150.0);
        assert!(chart.data[0].price != old_price);
        assert_abs_diff_eq!(chart.data[0].price, 150.0, epsilon = 1e-6);
    }

    #[test]
    fn test_variance() {
        let chart = PriceChart::default();
        let var = chart.variance();
        assert!(var > 0.0);
    }

    #[test]
    fn test_underlying_price() {
        let chart = PriceChart::default();
        let last_price = chart.data.last().unwrap().price;
        assert_abs_diff_eq!(chart.underlying_price(), last_price, epsilon = 1e-6);
    }
}
