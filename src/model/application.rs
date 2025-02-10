use chrono::NaiveDate;

use crate::model::chart::PriceChart;

pub struct OptiRust {
    pub chart: PriceChart
}

impl Default for OptiRust {
    fn default() -> OptiRust {
        OptiRust{chart: PriceChart::default()}
    }
}