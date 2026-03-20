extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplacePublisherPayout {
    publisher_id: String,
    payout_amount: u64,
    transactions: Vec<Transaction>,
}

impl MarketplacePublisherPayout {
    pub fn new(publisher_id: String, initial_payout: u64) -> Self {
        MarketplacePublisherPayout {
            publisher_id,
            payout_amount: initial_payout,
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        self.update_payout();
    }

    pub fn get_publisher_id(&self) -> &str {
        &self.publisher_id
    }

    pub fn get_total_transactions(&self) -> usize {
        self.transactions.len()
    }

    pub fn get_current_payout(&self) -> u64 {
        self.payout_amount
    }

    fn update_payout(&mut self) {
        let total_earned: u64 = self.transactions.iter().map(|t| t.amount).sum();
        self.payout_amount = total_earned;
    }
}

pub struct Transaction {
    transaction_id: String,
    amount: u64,
}

impl Transaction {
    pub fn new(transaction_id: String, amount: u64) -> Self {
        Transaction {
            transaction_id,
            amount,
        }
    }

    pub fn get_transaction_id(&self) -> &str {
        &self.transaction_id
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }
}
