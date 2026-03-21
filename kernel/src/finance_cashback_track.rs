extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct FinanceCashbackTrack {
    transactions: Vec<(String, u32)>, // (merchant_name, amount)
    total_cashback: u32,
}

impl FinanceCashbackTrack {
    pub fn new() -> Self {
        FinanceCashbackTrack {
            transactions: Vec::new(),
            total_cashback: 0,
        }
    }

    pub fn add_transaction(&mut self, merchant_name: String, amount: u32) {
        self.transactions.push((merchant_name, amount));
    }

    pub fn calculate_cashback(&mut self, rate: f32) -> u32 {
        let cashback = (self.total_transactions() as f32 * rate) as u32;
        self.total_cashback += cashback;
        cashback
    }

    pub fn total_transactions(&self) -> u32 {
        self.transactions.iter().map(|&(_, amount)| amount).sum()
    }

    pub fn get_transaction_count(&self) -> usize {
        self.transactions.len()
    }

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
        self.total_cashback = 0;
    }
}
