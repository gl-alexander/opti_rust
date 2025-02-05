use chrono::{Days, NaiveDate};
use core::f64;
use std::convert::TryInto;
use std::cmp::{min_by, max_by};

#[derive(Default)]
pub struct PriceChart {
    pub data: Vec<DataPoint>,
    pub squared_sum: f64,
    pub sum: f64,
    pub max_price: f64,
    pub min_price: f64,
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
        }
    }

    pub fn from_prices_and_date(input_data: Vec<f64>, starting_date: NaiveDate) -> PriceChart {
        let data_points: Vec<DataPoint> = input_data
        .iter()
        .map(|&p| DataPoint::new(p, starting_date.checked_add_days(Days::new(1)).unwrap())).collect();
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
}
