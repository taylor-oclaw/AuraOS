extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskPurchaseOrder {
    items: Vec<String>,
    quantities: Vec<u32>,
    prices: Vec<f64>,
}

impl AutoTaskPurchaseOrder {
    pub fn new() -> Self {
        AutoTaskPurchaseOrder {
            items: Vec::new(),
            quantities: Vec::new(),
            prices: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: String, quantity: u32, price: f64) {
        self.items.push(item);
        self.quantities.push(quantity);
        self.prices.push(price);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<(String, u32, f64)> {
        if index < self.items.len() {
            Some((
                self.items.remove(index),
                self.quantities.remove(index),
                self.prices.remove(index),
            ))
        } else {
            None
        }
    }

    pub fn get_total_cost(&self) -> f64 {
        self.items.iter().zip(self.quantities.iter()).zip(self.prices.iter())
            .map(|((_, &quantity), &price)| quantity as f64 * price)
            .sum()
    }

    pub fn list_items(&self) -> Vec<(String, u32, f64)> {
        self.items.iter().zip(self.quantities.iter()).zip(self.prices.iter())
            .map(|((item, &quantity), &price)| (item.clone(), quantity, price))
            .collect()
    }

    pub fn clear_order(&mut self) {
        self.items.clear();
        self.quantities.clear();
        self.prices.clear();
    }
}
