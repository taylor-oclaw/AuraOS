extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FinancePriceAlert {
    alerts: Vec<(String, f64)>,
}

impl FinancePriceAlert {
    pub fn new() -> Self {
        FinancePriceAlert { alerts: Vec::new() }
    }

    pub fn add_alert(&mut self, symbol: String, price: f64) {
        self.alerts.push((symbol, price));
    }

    pub fn remove_alert(&mut self, symbol: &str) {
        self.alerts.retain(|(s, _)| s != symbol);
    }

    pub fn get_alerts(&self) -> Vec<(String, f64)> {
        self.alerts.clone()
    }

    pub fn check_price(&self, symbol: &str, current_price: f64) -> bool {
        for (s, price) in &self.alerts {
            if s == symbol && current_price >= *price {
                return true;
            }
        }
        false
    }

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }
}
