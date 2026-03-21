extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FinanceExpenseReport {
    entries: Vec<ExpenseEntry>,
}

impl FinanceExpenseReport {
    pub fn new() -> Self {
        FinanceExpenseReport {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, description: String, amount: u32) {
        let entry = ExpenseEntry { description, amount };
        self.entries.push(entry);
    }

    pub fn total_expenses(&self) -> u32 {
        self.entries.iter().map(|entry| entry.amount).sum()
    }

    pub fn get_entries(&self) -> &Vec<ExpenseEntry> {
        &self.entries
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<ExpenseEntry> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

    pub fn find_entries_by_description(&self, description: &str) -> Vec<&ExpenseEntry> {
        self.entries.iter().filter(|entry| entry.description.contains(description)).collect()
    }
}

#[derive(Debug)]
pub struct ExpenseEntry {
    description: String,
    amount: u32,
}
