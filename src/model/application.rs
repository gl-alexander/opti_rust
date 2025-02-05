use chrono::NaiveDate;

use crate::model::chart::PriceChart;

pub struct OptiRust {
    pub chart: PriceChart
}

impl Default for OptiRust {
    fn default() -> OptiRust {
        let price_chart = PriceChart::from_prices_and_date(
            vec!(113.0, 116.5, 137.0, 145.3, 101.32), 
            NaiveDate::from_ymd_opt(2025, 1, 3).unwrap());
        OptiRust{chart: price_chart}
    }
}