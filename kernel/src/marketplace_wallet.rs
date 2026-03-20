extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceWallet {
    balance: u64,
    transactions: Vec<String>,
}

impl MarketplaceWallet {
    pub fn new(initial_balance: u64) -> Self {
        MarketplaceWallet {
            balance: initial_balance,
            transactions: Vec::new(),
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn deposit(&mut self, amount: u64) -> bool {
        if amount == 0 {
            return false;
        }
        self.balance += amount;
        let transaction = format!("Deposited {} units", amount);
        self.transactions.push(transaction);
        true
    }

    pub fn withdraw(&mut self, amount: u64) -> bool {
        if amount == 0 || amount > self.balance {
            return false;
        }
        self.balance -= amount;
        let transaction = format!("Withdrew {} units", amount);
        self.transactions.push(transaction);
        true
    }

    pub fn transfer(&mut self, recipient: &str, amount: u64) -> bool {
        if amount == 0 || amount > self.balance {
            return false;
        }
        self.balance -= amount;
        let transaction = format!("Transferred {} units to {}", amount, recipient);
        self.transactions.push(transaction);
        true
    }

    pub fn get_transaction_history(&self) -> &Vec<String> {
        &self.transactions
    }
}
