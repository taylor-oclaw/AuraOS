extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FinanceTracker {
    transactions: Vec<(String, i32)>,
}

impl FinanceTracker {
    pub fn new() -> Self {
        FinanceTracker {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, description: String, amount: i32) {
        self.transactions.push((description, amount));
    }

    pub fn get_total_balance(&self) -> i32 {
        self.transactions.iter().map(|&(_, amount)| amount).sum()
    }

    pub fn list_transactions(&self) -> Vec<(String, i32)> {
        self.transactions.clone()
    }

    pub fn count_transactions(&self) -> usize {
        self.transactions.len()
    }

    pub fn get_largest_transaction(&self) -> Option<(String, i32)> {
        self.transactions.iter().max_by_key(|&(_, amount)| amount).cloned()
    }
}
