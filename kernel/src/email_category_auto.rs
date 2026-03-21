extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailCategoryAuto {
    categories: Vec<String>,
}

impl EmailCategoryAuto {
    pub fn new() -> Self {
        EmailCategoryAuto {
            categories: Vec::new(),
        }
    }

    pub fn add_category(&mut self, category: &str) {
        if !self.categories.contains(&String::from(category)) {
            self.categories.push(String::from(category));
        }
    }

    pub fn remove_category(&mut self, category: &str) {
        self.categories.retain(|cat| cat != category);
    }

    pub fn list_categories(&self) -> Vec<String> {
        self.categories.clone()
    }

    pub fn categorize_email(&self, email_content: &str) -> Option<&String> {
        for category in &self.categories {
            if email_content.contains(category) {
                return Some(category);
            }
        }
        None
    }

    pub fn update_category(&mut self, old_category: &str, new_category: &str) {
        if let Some(index) = self.categories.iter().position(|cat| cat == old_category) {
            self.categories[index] = String::from(new_category);
        }
    }
}
