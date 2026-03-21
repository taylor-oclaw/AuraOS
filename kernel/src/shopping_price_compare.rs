extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct ShoppingPriceCompare {
    items: Vec<(String, f64)>,
}

impl ShoppingPriceCompare {
    pub fn new() -> Self {
        ShoppingPriceCompare { items: Vec::new() }
    }

    pub fn add_item(&mut self, name: String, price: f64) {
        self.items.push((name, price));
    }

    pub fn remove_item(&mut self, name: &str) {
        self.items.retain(|(item_name, _)| item_name != name);
    }

    pub fn get_price(&self, name: &str) -> Option<f64> {
        self.items.iter().find_map(|(item_name, price)| {
            if item_name == name {
                Some(*price)
            } else {
                None
            }
        })
    }

    pub fn get_cheapest_item(&self) -> Option<&(String, f64)> {
        self.items.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(core::cmp::Ordering::Equal))
    }

    pub fn get_most_expensive_item(&self) -> Option<&(String, f64)> {
        self.items.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(core::cmp::Ordering::Equal))
    }
}
