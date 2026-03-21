extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct MarketplacePayoutScheduler {
    payouts: Vec<(String, u64)>,
}

impl MarketplacePayoutScheduler {
    pub fn new() -> Self {
        MarketplacePayoutScheduler {
            payouts: Vec::new(),
        }
    }

    pub fn add_payout(&mut self, user_id: String, amount: u64) {
        self.payouts.push((user_id, amount));
    }

    pub fn remove_payout(&mut self, user_id: &str) -> Option<u64> {
        let index = self.payouts.iter().position(|(id, _)| id == user_id);
        if let Some(idx) = index {
            return Some(self.payouts.remove(idx).1);
        }
        None
    }

    pub fn get_payout(&self, user_id: &str) -> Option<u64> {
        self.payouts.iter().find(|(id, _)| id == user_id).map(|(_, amount)| *amount)
    }

    pub fn total_payouts(&self) -> u64 {
        self.payouts.iter().map(|(_, amount)| amount).sum()
    }

    pub fn list_all_payouts(&self) -> Vec<(String, u64)> {
        self.payouts.clone()
    }
}
