extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct FinanceSpendingAlert {
    budget: u32,
    expenses: Vec<u32>,
    alerts_sent: bool,
}

impl FinanceSpendingAlert {
    pub fn new(budget: u32) -> Self {
        FinanceSpendingAlert {
            budget,
            expenses: Vec::new(),
            alerts_sent: false,
        }
    }

    pub fn add_expense(&mut self, amount: u32) {
        self.expenses.push(amount);
    }

    pub fn total_spent(&self) -> u32 {
        self.expenses.iter().sum()
    }

    pub fn remaining_budget(&self) -> u32 {
        self.budget.saturating_sub(self.total_spent())
    }

    pub fn check_alerts(&mut self) {
        if !self.alerts_sent && self.remaining_budget() == 0 {
            // Send alert logic here
            self.alerts_sent = true;
        }
    }

    pub fn reset_alerts(&mut self) {
        self.alerts_sent = false;
    }
}
