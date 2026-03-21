extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct CompanyUsageMeter {
    company_name: String,
    usage_data: Vec<(u32, u64)>, // (timestamp, usage_amount)
}

impl CompanyUsageMeter {
    pub fn new(company_name: &str) -> Self {
        CompanyUsageMeter {
            company_name: String::from(company_name),
            usage_data: Vec::new(),
        }
    }

    pub fn record_usage(&mut self, timestamp: u32, amount: u64) {
        self.usage_data.push((timestamp, amount));
    }

    pub fn get_total_usage(&self) -> u64 {
        self.usage_data.iter().map(|&(_, amount)| amount).sum()
    }

    pub fn get_average_usage(&self) -> f64 {
        if self.usage_data.is_empty() {
            0.0
        } else {
            self.get_total_usage() as f64 / self.usage_data.len() as f64
        }
    }

    pub fn get_latest_usage(&self) -> Option<u64> {
        self.usage_data.last().map(|&(_, amount)| amount)
    }

    pub fn clear_usage_data(&mut self) {
        self.usage_data.clear();
    }
}
