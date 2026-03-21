extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshComputeCredits {
    credits: u32,
    max_credits: u32,
    used_credits: u32,
    credit_history: Vec<u32>,
}

impl MeshComputeCredits {
    pub fn new(max_credits: u32) -> Self {
        MeshComputeCredits {
            credits: max_credits,
            max_credits,
            used_credits: 0,
            credit_history: Vec::new(),
        }
    }

    pub fn add_credits(&mut self, amount: u32) {
        if amount > 0 {
            self.credits += amount;
            if self.credits > self.max_credits {
                self.credits = self.max_credits;
            }
            self.credit_history.push(amount);
        }
    }

    pub fn use_credits(&mut self, amount: u32) -> bool {
        if amount > 0 && amount <= self.credits {
            self.credits -= amount;
            self.used_credits += amount;
            self.credit_history.push(amount);
            true
        } else {
            false
        }
    }

    pub fn get_remaining_credits(&self) -> u32 {
        self.credits
    }

    pub fn get_credit_history(&self) -> &Vec<u32> {
        &self.credit_history
    }

    pub fn reset_used_credits(&mut self) {
        self.used_credits = 0;
    }
}
