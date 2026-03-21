extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct InvestmentTracker {
    investments: Vec<(String, u32)>, // (asset_name, amount)
    total_investment: u32,
}

impl InvestmentTracker {
    pub fn new() -> Self {
        InvestmentTracker {
            investments: Vec::new(),
            total_investment: 0,
        }
    }

    pub fn add_investment(&mut self, asset_name: String, amount: u32) {
        self.investments.push((asset_name, amount));
        self.total_investment += amount;
    }

    pub fn remove_investment(&mut self, asset_name: &str) -> Option<u32> {
        if let Some(index) = self.investments.iter().position(|(name, _)| name == asset_name) {
            let (_, amount) = self.investments.remove(index);
            self.total_investment -= amount;
            Some(amount)
        } else {
            None
        }
    }

    pub fn get_total_investment(&self) -> u32 {
        self.total_investment
    }

    pub fn list_investments(&self) -> Vec<(String, u32)> {
        self.investments.clone()
    }

    pub fn find_investment(&self, asset_name: &str) -> Option<u32> {
        self.investments.iter().find_map(|(name, amount)| {
            if name == asset_name {
                Some(*amount)
            } else {
                None
            }
        }
    }
}
