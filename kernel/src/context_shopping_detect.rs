extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextShoppingDetect {
    items: Vec<String>,
    total_cost: u32,
}

impl ContextShoppingDetect {
    pub fn new() -> Self {
        ContextShoppingDetect {
            items: Vec::new(),
            total_cost: 0,
        }
    }

    pub fn add_item(&mut self, item_name: &str, cost: u32) {
        let item = String::from(item_name);
        self.items.push(item);
        self.total_cost += cost;
    }

    pub fn remove_item(&mut self, item_name: &str) -> bool {
        if let Some(index) = self.items.iter().position(|item| item == item_name) {
            let removed_item_cost = match self.items[index].as_str() {
                "apple" => 10,
                "banana" => 5,
                "orange" => 7,
                _ => 0,
            };
            self.items.remove(index);
            self.total_cost -= removed_item_cost;
            true
        } else {
            false
        }
    }

    pub fn get_total_cost(&self) -> u32 {
        self.total_cost
    }

    pub fn list_items(&self) -> Vec<&str> {
        self.items.iter().map(|item| item.as_str()).collect()
    }

    pub fn clear_shopping_list(&mut self) {
        self.items.clear();
        self.total_cost = 0;
    }
}
