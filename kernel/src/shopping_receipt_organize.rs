extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ShoppingReceipt {
    items: Vec<Item>,
}

impl ShoppingReceipt {
    pub fn new() -> Self {
        ShoppingReceipt { items: Vec::new() }
    }

    pub fn add_item(&mut self, name: String, price: u32, quantity: u32) {
        let item = Item { name, price, quantity };
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_total_price(&self) -> u32 {
        self.items.iter().map(|item| item.price * item.quantity).sum()
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.iter().map(|item| String::from("info")).collect()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

struct Item {
    name: String,
    price: u32,
    quantity: u32,
}
