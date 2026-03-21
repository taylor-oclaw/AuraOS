extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceTaxReporting {
    transactions: Vec<Transaction>,
}

impl MarketplaceTaxReporting {
    pub fn new() -> Self {
        MarketplaceTaxReporting {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn total_sales(&self) -> u32 {
        self.transactions.iter().map(|t| t.amount).sum()
    }

    pub fn calculate_tax(&self, tax_rate: f32) -> f32 {
        (self.total_sales() as f32 * tax_rate) / 100.0
    }

    pub fn list_transactions(&self) -> Vec<String> {
        self.transactions.iter().map(|t| t.to_string()).collect()
    }

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }
}

#[derive(Debug)]
pub struct Transaction {
    id: u32,
    amount: u32,
    description: String,
}

impl Transaction {
    pub fn new(id: u32, amount: u32, description: String) -> Self {
        Transaction { id, amount, description }
    }

    pub fn to_string(&self) -> String {
        format!("Transaction ID: {}, Amount: ${}, Description: {}", self.id, self.amount, self.description)
    }
}
