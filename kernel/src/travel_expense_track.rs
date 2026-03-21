extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelExpenseTrack {
    expenses: Vec<(String, u32)>,
}

impl TravelExpenseTrack {
    pub fn new() -> Self {
        TravelExpenseTrack {
            expenses: Vec::new(),
        }
    }

    pub fn add_expense(&mut self, description: String, amount: u32) {
        self.expenses.push((description, amount));
    }

    pub fn total_expenses(&self) -> u32 {
        self.expenses.iter().map(|&(_, amount)| amount).sum()
    }

    pub fn list_expenses(&self) -> Vec<(String, u32)> {
        self.expenses.clone()
    }

    pub fn remove_expense(&mut self, index: usize) -> Option<(String, u32)> {
        if index < self.expenses.len() {
            Some(self.expenses.remove(index))
        } else {
            None
        }
    }

    pub fn find_expense_by_description(&self, description: &str) -> Option<&(String, u32)> {
        self.expenses.iter().find(|&&(ref desc, _)| desc == description)
    }
}
