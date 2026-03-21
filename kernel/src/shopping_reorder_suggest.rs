extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ShoppingReorderSuggest {
    items: Vec<String>,
    reorder_threshold: usize,
}

impl ShoppingReorderSuggest {
    pub fn new(reorder_threshold: usize) -> Self {
        ShoppingReorderSuggest {
            items: Vec::new(),
            reorder_threshold,
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) {
        if let Some(index) = self.items.iter().position(|i| i == item) {
            self.items.remove(index);
        }
    }

    pub fn get_items(&self) -> &[String] {
        &self.items
    }

    pub fn needs_reorder(&self) -> bool {
        self.items.len() < self.reorder_threshold
    }

    pub fn suggest_reorder(&self) -> Vec<String> {
        if self.needs_reorder() {
            self.items.clone()
        } else {
            Vec::new()
        }
    }
}
