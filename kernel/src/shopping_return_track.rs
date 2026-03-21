extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut tracker = ShoppingReturnTrack::new();
    tracker.add_item("Laptop", 12345);
    tracker.add_item("Smartphone", 67890);
    tracker.mark_returned(12345);

    loop {}
}

pub struct ShoppingReturnTrack {
    items: Vec<(String, u32, bool)>, // (item_name, item_id, is_returned)
}

impl ShoppingReturnTrack {
    pub fn new() -> Self {
        ShoppingReturnTrack { items: Vec::new() }
    }

    pub fn add_item(&mut self, name: &str, id: u32) {
        self.items.push((String::from(name), id, false));
    }

    pub fn mark_returned(&mut self, item_id: u32) -> bool {
        for item in self.items.iter_mut() {
            if item.1 == item_id {
                item.2 = true;
                return true;
            }
        }
        false
    }

    pub fn is_item_returned(&self, item_id: u32) -> Option<bool> {
        for item in &self.items {
            if item.1 == item_id {
                return Some(item.2);
            }
        }
        None
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn get_returned_items(&self) -> Vec<String> {
        self.items.iter()
            .filter(|item| item.2)
            .map(|item| item.0.clone())
            .collect()
    }
}
