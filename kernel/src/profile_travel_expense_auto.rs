extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct TravelExpenseAuto {
    expenses: Vec<(String, u32)>,
    total_spent: u32,
    budget: u32,
}

impl TravelExpenseAuto {
    pub fn new(budget: u32) -> Self {
        TravelExpenseAuto {
            expenses: Vec::new(),
            total_spent: 0,
            budget,
        }
    }

    pub fn add_expense(&mut self, description: String, amount: u32) -> bool {
        if self.total_spent + amount > self.budget {
            false
        } else {
            self.expenses.push((description, amount));
            self.total_spent += amount;
            true
        }
    }

    pub fn get_total_spent(&self) -> u32 {
        self.total_spent
    }

    pub fn get_budget(&self) -> u32 {
        self.budget
    }

    pub fn get_remaining_budget(&self) -> u32 {
        self.budget - self.total_spent
    }

    pub fn list_expenses(&self) -> Vec<(String, u32)> {
        self.expenses.clone()
    }
}
