extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceFeatured {
    items: Vec<String>,
}

impl MarketplaceFeatured {
    pub fn new() -> Self {
        MarketplaceFeatured { items: Vec::new() }
    }

    pub fn add_item(&mut self, item_name: &str) {
        self.items.push(item_name.to_string());
    }

    pub fn remove_item(&mut self, item_name: &str) {
        if let Some(index) = self.items.iter().position(|x| x == item_name) {
            self.items.remove(index);
        }
    }

    pub fn get_items(&self) -> &[String] {
        &self.items
    }

    pub fn is_item_featured(&self, item_name: &str) -> bool {
        self.items.contains(&item_name.to_string())
    }

    pub fn clear_all_items(&mut self) {
        self.items.clear();
    }
}
