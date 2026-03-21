extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct GiftBudgetTracker {
    budget: u32,
    gifts: Vec<String>,
}

impl GiftBudgetTracker {
    pub fn new(budget: u32) -> Self {
        GiftBudgetTracker {
            budget,
            gifts: Vec::new(),
        }
    }

    pub fn add_gift(&mut self, gift_name: &str, cost: u32) -> Result<(), String> {
        if cost > self.budget {
            Err(String::from("Insufficient budget"))
        } else {
            self.gifts.push(gift_name.to_string());
            self.budget -= cost;
            Ok(())
        }
    }

    pub fn remove_gift(&mut self, gift_name: &str) -> Result<u32, String> {
        if let Some(index) = self.gifts.iter().position(|g| g == gift_name) {
            let removed_gift = self.gifts.remove(index);
            // Assuming we have a way to get the cost of the removed gift
            let cost = 10; // Placeholder for actual cost retrieval logic
            self.budget += cost;
            Ok(cost)
        } else {
            Err(String::from("Gift not found"))
        }
    }

    pub fn list_gifts(&self) -> Vec<String> {
        self.gifts.clone()
    }

    pub fn get_budget(&self) -> u32 {
        self.budget
    }

    pub fn total_spent(&self, initial_budget: u32) -> u32 {
        initial_budget - self.budget
    }
}
