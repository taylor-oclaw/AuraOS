extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct MarketplaceTrending {
    items: Vec<String>,
    trending_items: Vec<String>,
}

impl MarketplaceTrending {
    pub fn new() -> Self {
        MarketplaceTrending {
            items: Vec::new(),
            trending_items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item_name: &str) {
        let item = String::from(item_name);
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item_name: &str) -> bool {
        if let Some(index) = self.items.iter().position(|x| x == item_name) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_trending(&mut self) {
        // Simple logic to determine trending items (for example, top 3 most added)
        let mut sorted_items = self.items.clone();
        sorted_items.sort_by_key(|item| -self.items.iter().filter(|&x| x == item).count() as isize);
        self.trending_items = sorted_items.into_iter().take(3).collect();
    }

    pub fn get_trending(&self) -> Vec<String> {
        self.trending_items.clone()
    }

    pub fn list_all_items(&self) -> Vec<String> {
        self.items.clone()
    }
}
