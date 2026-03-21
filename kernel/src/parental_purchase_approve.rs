extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn parental_purchase_approve_init() {
    // Initialization logic for the module
}

pub extern "C" fn parental_purchase_approve_exit() {
    // Cleanup logic for the module
}

pub struct ParentalPurchaseApprover {
    approved_items: Vec<String>,
    blocked_items: Vec<String>,
}

impl ParentalPurchaseApprover {
    pub fn new() -> Self {
        ParentalPurchaseApprover {
            approved_items: Vec::new(),
            blocked_items: Vec::new(),
        }
    }

    pub fn add_approved_item(&mut self, item: String) {
        if !self.approved_items.contains(&item) {
            self.approved_items.push(item);
        }
    }

    pub fn remove_approved_item(&mut self, item: &str) {
        if let Some(index) = self.approved_items.iter().position(|x| x == item) {
            self.approved_items.remove(index);
        }
    }

    pub fn add_blocked_item(&mut self, item: String) {
        if !self.blocked_items.contains(&item) {
            self.blocked_items.push(item);
        }
    }

    pub fn remove_blocked_item(&mut self, item: &str) {
        if let Some(index) = self.blocked_items.iter().position(|x| x == item) {
            self.blocked_items.remove(index);
        }
    }

    pub fn can_purchase(&self, item: &str) -> bool {
        !self.blocked_items.contains(item) && (self.approved_items.is_empty() || self.approved_items.contains(item))
    }
}
