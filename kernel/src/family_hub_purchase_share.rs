extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

pub struct FamilyHubPurchaseShare {
    items: Vec<String>,
    prices: Vec<u32>,
    total_spent: u32,
    budget: u32,
}

impl FamilyHubPurchaseShare {
    pub fn new(budget: u32) -> Self {
        FamilyHubPurchaseShare {
            items: Vec::new(),
            prices: Vec::new(),
            total_spent: 0,
            budget,
        }
    }

    pub fn add_item(&mut self, item: String, price: u32) -> bool {
        if self.total_spent + price <= self.budget {
            self.items.push(item);
            self.prices.push(price);
            self.total_spent += price;
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.items.iter().position(|i| i == item) {
            self.total_spent -= self.prices[index];
            self.items.remove(index);
            self.prices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_total_spent(&self) -> u32 {
        self.total_spent
    }

    pub fn get_budget(&self) -> u32 {
        self.budget
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.clone()
    }
}
