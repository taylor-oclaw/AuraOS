extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn marketplace_core_init() {
    // Initialization logic for the marketplace core module
}

#[no_mangle]
pub extern "C" fn marketplace_core_exit() {
    // Cleanup logic for the marketplace core module
}

pub struct MarketplaceCore {
    items: Vec<String>,
}

impl MarketplaceCore {
    pub fn new() -> Self {
        MarketplaceCore { items: Vec::new() }
    }

    pub fn add_item(&mut self, item_name: &str) {
        self.items.push(item_name.to_string());
    }

    pub fn remove_item(&mut self, item_name: &str) {
        if let Some(index) = self.items.iter().position(|x| x == item_name) {
            self.items.remove(index);
        }
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn has_item(&self, item_name: &str) -> bool {
        self.items.contains(&item_name.to_string())
    }

    pub fn count_items(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace_core() {
        let mut marketplace = MarketplaceCore::new();
        assert_eq!(marketplace.count_items(), 0);

        marketplace.add_item("AI Model");
        assert_eq!(marketplace.count_items(), 1);
        assert!(marketplace.has_item("AI Model"));

        marketplace.add_item("Machine Learning Library");
        assert_eq!(marketplace.count_items(), 2);
        assert!(marketplace.has_item("Machine Learning Library"));

        let items = marketplace.list_items();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&String::from("AI Model")));
        assert!(items.contains(&String::from("Machine Learning Library")));

        marketplace.remove_item("AI Model");
        assert_eq!(marketplace.count_items(), 1);
        assert!(!marketplace.has_item("AI Model"));

        marketplace.remove_item("Machine Learning Library");
        assert_eq!(marketplace.count_items(), 0);
        assert!(!marketplace.has_item("Machine Learning Library"));
    }
}
