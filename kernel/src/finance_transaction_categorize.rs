extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FinanceTransaction {
    pub id: u32,
    pub amount: f64,
    pub category: String,
    pub description: String,
}

impl FinanceTransaction {
    pub fn new(id: u32, amount: f64, category: &str, description: &str) -> Self {
        FinanceTransaction {
            id,
            amount,
            category: String::from(category),
            description: String::from(description),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_category(&self) -> &str {
        &self.category
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn update_category(&mut self, new_category: &str) {
        self.category = String::from(new_category);
    }
}

#[derive(Debug)]
pub struct FinanceTransactionCategorizer {
    transactions: Vec<FinanceTransaction>,
}

impl FinanceTransactionCategorizer {
    pub fn new() -> Self {
        FinanceTransactionCategorizer {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: FinanceTransaction) {
        self.transactions.push(transaction);
    }

    pub fn get_transactions_by_category(&self, category: &str) -> Vec<&FinanceTransaction> {
        self.transactions
            .iter()
            .filter(|t| t.get_category() == category)
            .collect()
    }

    pub fn total_amount_by_category(&self, category: &str) -> f64 {
        self.transactions
            .iter()
            .filter(|t| t.get_category() == category)
            .map(|t| t.get_amount())
            .sum()
    }

    pub fn update_transaction_category(&mut self, transaction_id: u32, new_category: &str) -> bool {
        for transaction in &mut self.transactions {
            if transaction.get_id() == transaction_id {
                transaction.update_category(new_category);
                return true;
            }
        }
        false
    }

    pub fn remove_transaction(&mut self, transaction_id: u32) -> bool {
        let pos = self.transactions.iter().position(|t| t.get_id() == transaction_id);
        if let Some(index) = pos {
            self.transactions.remove(index);
            true
        } else {
            false
        }
    }
}
