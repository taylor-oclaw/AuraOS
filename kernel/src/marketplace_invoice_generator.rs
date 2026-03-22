extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceInvoiceGenerator {
    items: Vec<Item>,
    total_amount: u32,
}

impl MarketplaceInvoiceGenerator {
    pub fn new() -> Self {
        MarketplaceInvoiceGenerator {
            items: Vec::new(),
            total_amount: 0,
        }
    }

    pub fn add_item(&mut self, name: String, price: u32) {
        let item = Item { name, price };
        self.items.push(item);
        self.total_amount += price;
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            let removed_item = self.items.remove(index);
            self.total_amount -= removed_item.price;
            Some(removed_item)
        } else {
            None
        }
    }

    pub fn get_total_amount(&self) -> u32 {
        self.total_amount
    }

    pub fn list_items(&self) -> Vec<&Item> {
        self.items.iter().collect()
    }

    pub fn clear_invoice(&mut self) {
        self.items.clear();
        self.total_amount = 0;
    }
}

struct Item {
    name: String,
    price: u32,
}