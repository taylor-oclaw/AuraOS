extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplacePublisherPortal {
    // Example fields for the marketplace publisher portal
    items: Vec<String>,
    categories: Vec<String>,
}

impl MarketplacePublisherPortal {
    pub fn new() -> Self {
        MarketplacePublisherPortal {
            items: Vec::new(),
            categories: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item_name: &str) {
        self.items.push(item_name.to_string());
    }

    pub fn remove_item(&mut self, item_name: &str) -> bool {
        if let Some(index) = self.items.iter().position(|x| x == item_name) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn add_category(&mut self, category_name: &str) {
        self.categories.push(category_name.to_string());
    }

    pub fn remove_category(&mut self, category_name: &str) -> bool {
        if let Some(index) = self.categories.iter().position(|x| x == category_name) {
            self.categories.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_categories(&self) -> Vec<String> {
        self.categories.clone()
    }
}
