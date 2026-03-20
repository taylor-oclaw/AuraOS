extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceCategories {
    categories: Vec<String>,
}

impl MarketplaceCategories {
    pub fn new() -> Self {
        MarketplaceCategories {
            categories: Vec::new(),
        }
    }

    pub fn add_category(&mut self, category_name: &str) {
        if !self.categories.contains(&String::from(category_name)) {
            self.categories.push(String::from(category_name));
        }
    }

    pub fn remove_category(&mut self, category_name: &str) {
        self.categories.retain(|cat| cat != category_name);
    }

    pub fn get_categories(&self) -> Vec<String> {
        self.categories.clone()
    }

    pub fn has_category(&self, category_name: &str) -> bool {
        self.categories.contains(&String::from(category_name))
    }

    pub fn count_categories(&self) -> usize {
        self.categories.len()
    }
}
