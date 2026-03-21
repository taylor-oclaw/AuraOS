extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct FinanceBankConnect {
    account_name: String,
    balance: u64,
    transactions: Vec<String>,
}

impl FinanceBankConnect {
    pub fn new(account_name: &str, initial_balance: u64) -> Self {
        FinanceBankConnect {
            account_name: String::from(account_name),
            balance: initial_balance,
            transactions: Vec::new(),
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        if amount > 0 {
            self.balance += amount;
            let transaction = String::from("info");
            self.transactions.push(transaction);
        }
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        if amount == 0 {
            return Err(String::from("Amount cannot be zero"));
        }
        if amount > self.balance {
            return Err(String::from("Insufficient funds"));
        }
        self.balance -= amount;
        let transaction = String::from("info");
        self.transactions.push(transaction);
        Ok(())
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn get_account_name(&self) -> &str {
        &self.account_name
    }

    pub fn transaction_history(&self) -> &Vec<String> {
        &self.transactions
    }
}
