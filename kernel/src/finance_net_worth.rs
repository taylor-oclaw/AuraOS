extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct FinanceNetWorth {
    assets: Vec<(String, u64)>,
    liabilities: Vec<(String, u64)>,
}

impl FinanceNetWorth {
    pub fn new() -> Self {
        FinanceNetWorth {
            assets: Vec::new(),
            liabilities: Vec::new(),
        }
    }

    pub fn add_asset(&mut self, name: String, value: u64) {
        self.assets.push((name, value));
    }

    pub fn add_liability(&mut self, name: String, value: u64) {
        self.liabilities.push((name, value));
    }

    pub fn total_assets(&self) -> u64 {
        self.assets.iter().map(|&(_, value)| value).sum()
    }

    pub fn total_liabilities(&self) -> u64 {
        self.liabilities.iter().map(|&(_, value)| value).sum()
    }

    pub fn net_worth(&self) -> i64 {
        let assets = self.total_assets();
        let liabilities = self.total_liabilities();
        assets as i64 - liabilities as i64
    }
}
