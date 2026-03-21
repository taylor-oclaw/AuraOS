extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut tracker = DebtSocialTracker::new();
    tracker.add_debt("Alice", 100);
    tracker.add_debt("Bob", 200);
    tracker.pay_debt("Alice", 50);
    println!("Total debt: {}", tracker.total_debt());
    println!("Debt of Bob: {}", tracker.debt_of("Bob"));
    loop {}
}

pub struct DebtSocialTracker {
    debts: Vec<(String, u32)>,
}

impl DebtSocialTracker {
    pub fn new() -> Self {
        DebtSocialTracker { debts: Vec::new() }
    }

    pub fn add_debt(&mut self, name: &str, amount: u32) {
        for (n, a) in self.debts.iter_mut() {
            if n == name {
                *a += amount;
                return;
            }
        }
        self.debts.push((String::from(name), amount));
    }

    pub fn pay_debt(&mut self, name: &str, amount: u32) -> bool {
        for (n, a) in self.debts.iter_mut() {
            if n == name && *a >= amount {
                *a -= amount;
                return true;
            }
        }
        false
    }

    pub fn total_debt(&self) -> u32 {
        self.debts.iter().map(|(_, a)| a).sum()
    }

    pub fn debt_of(&self, name: &str) -> u32 {
        for (n, a) in self.debts.iter() {
            if n == name {
                return *a;
            }
        }
        0
    }

    pub fn list_debts(&self) -> Vec<(String, u32)> {
        self.debts.clone()
    }
}
