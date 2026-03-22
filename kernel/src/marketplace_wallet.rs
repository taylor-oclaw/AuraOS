#![no_std]
#![feature(allocator_api)]
#![feature(const_mut_refs)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

struct MarketplaceWallet {
    balance: u64,
    transactions: Vec<String>,
}

impl MarketplaceWallet {
    pub fn new() -> Self {
        MarketplaceWallet {
            balance: 0,
            transactions: Vec::new(),
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        self.transactions.push(format!("Deposited {} tokens", amount));
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        self.transactions.push(format!("Withdrew {} tokens", amount));
        Ok(())
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn get_transactions(&self) -> Vec<String> {
        self.transactions.clone()
    }
}

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    let mut wallet = MarketplaceWallet::new();
    wallet.deposit(100);
    println!("Initial balance: {}", wallet.get_balance());
    for transaction in wallet.get_transactions() {
        println!("{}", transaction);
    }
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}