extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn gift_professional_occasion_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn gift_professional_occasion_exit() {
    // Cleanup logic for the module
}

pub struct GiftProfessionalOccasion {
    items: Vec<String>,
    budget: u32,
}

impl GiftProfessionalOccasion {
    pub fn new(budget: u32) -> Self {
        GiftProfessionalOccasion {
            items: Vec::new(),
            budget: budget,
        }
    }

    pub fn add_item(&mut self, item: String) {
        if self.budget > 0 {
            self.items.push(item);
        }
    }

    pub fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_items(&self) -> &Vec<String> {
        &self.items
    }

    pub fn check_budget(&self) -> u32 {
        self.budget
    }

    pub fn total_items(&self) -> usize {
        self.items.len()
    }
}
