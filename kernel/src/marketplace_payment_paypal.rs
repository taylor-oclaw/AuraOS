extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplacePaymentPaypal {
    api_key: String,
    transactions: Vec<Transaction>,
}

impl MarketplacePaymentPaypal {
    pub fn new(api_key: &str) -> Self {
        MarketplacePaymentPaypal {
            api_key: String::from(api_key),
            transactions: Vec::new(),
        }
    }

    pub fn process_payment(&mut self, amount: u32, currency: &str, user_id: &str) -> Result<(), &'static str> {
        if amount == 0 {
            return Err("Amount cannot be zero");
        }
        let transaction = Transaction {
            amount,
            currency: String::from(currency),
            user_id: String::from(user_id),
            status: String::from("Pending"),
        };
        self.transactions.push(transaction);
        Ok(())
    }

    pub fn get_transaction_status(&self, transaction_id: usize) -> Option<&str> {
        self.transactions.get(transaction_id).map(|t| t.status.as_str())
    }

    pub fn refund_payment(&mut self, transaction_id: usize) -> Result<(), &'static str> {
        if let Some(transaction) = self.transactions.get_mut(transaction_id) {
            if transaction.status == "Pending" || transaction.status == "Completed" {
                transaction.status = String::from("Refunded");
                Ok(())
            } else {
                Err("Transaction cannot be refunded")
            }
        } else {
            Err("Transaction not found")
        }
    }

    pub fn cancel_payment(&mut self, transaction_id: usize) -> Result<(), &'static str> {
        if let Some(transaction) = self.transactions.get_mut(transaction_id) {
            if transaction.status == "Pending" {
                transaction.status = String::from("Cancelled");
                Ok(())
            } else {
                Err("Transaction cannot be cancelled")
            }
        } else {
            Err("Transaction not found")
        }
    }

    pub fn list_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

struct Transaction {
    amount: u32,
    currency: String,
    user_id: String,
    status: String,
}
