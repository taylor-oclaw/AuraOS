extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ShoppingPriceHistory {
    items: Vec<(String, u32)>,
}

impl ShoppingPriceHistory {
    pub fn new() -> Self {
        ShoppingPriceHistory { items: Vec::new() }
    }

    pub fn add_item(&mut self, item_name: String, price: u32) {
        self.items.push((item_name, price));
    }

    pub fn get_price(&self, item_name: &str) -> Option<u32> {
        for (name, price) in &self.items {
            if name == item_name {
                return Some(*price);
            }
        }
        None
    }

    pub fn update_price(&mut self, item_name: &str, new_price: u32) -> bool {
        for (name, price) in &mut self.items {
            if name == item_name {
                *price = new_price;
                return true;
            }
        }
        false
    }

    pub fn remove_item(&mut self, item_name: &str) -> bool {
        let pos = self.items.iter().position(|(name, _)| name == item_name);
        if let Some(index) = pos {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_items(&self) -> Vec<(String, u32)> {
        self.items.clone()
    }
}
