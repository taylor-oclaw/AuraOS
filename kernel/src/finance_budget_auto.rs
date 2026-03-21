extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FinanceBudgetAuto {
    budget: u32,
    expenses: Vec<u32>,
    income: Vec<u32>,
}

impl FinanceBudgetAuto {
    pub fn new(budget: u32) -> Self {
        FinanceBudgetAuto {
            budget,
            expenses: Vec::new(),
            income: Vec::new(),
        }
    }

    pub fn add_expense(&mut self, amount: u32) {
        if amount <= self.budget {
            self.expenses.push(amount);
            self.budget -= amount;
        } else {
            // Handle error: expense exceeds budget
        }
    }

    pub fn add_income(&mut self, amount: u32) {
        self.income.push(amount);
        self.budget += amount;
    }

    pub fn total_expenses(&self) -> u32 {
        self.expenses.iter().sum()
    }

    pub fn total_income(&self) -> u32 {
        self.income.iter().sum()
    }

    pub fn remaining_budget(&self) -> u32 {
        self.budget
    }
}
