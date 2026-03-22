extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn marketplace_free_tier_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn marketplace_free_tier_exit() {
    // Cleanup logic for the module
}

pub struct MarketplaceFreeTier {
    items: Vec<String>,
    max_items: usize,
}

impl MarketplaceFreeTier {
    pub fn new(max_items: usize) -> Self {
        MarketplaceFreeTier {
            items: Vec::new(),
            max_items,
        }
    }

    pub fn add_item(&mut self, item_name: &str) -> bool {
        if self.items.len() < self.max_items {
            self.items.push(item_name.to_string());
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, item_name: &str) -> bool {
        let position = self.items.iter().position(|x| x == item_name);
        if let Some(index) = position {
            self.items.remove(index);
            true
        } else {
            false
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
    fn test_marketplace_free_tier() {
        let mut marketplace = MarketplaceFreeTier::new(3);

        assert!(marketplace.add_item("AI Model 1"));
        assert!(marketplace.add_item("AI Model 2"));
        assert!(!marketplace.add_item("AI Model 4")); // Should fail, max_items is 3

        assert_eq!(marketplace.count_items(), 2);
        assert!(marketplace.has_item("AI Model 1"));
        assert!(!marketplace.has_item("AI Model 3"));

        let items = marketplace.list_items();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&"AI Model 1".to_string()));
        assert!(items.contains(&"AI Model 2".to_string()));

        assert!(marketplace.remove_item("AI Model 1"));
        assert!(!marketplace.has_item("AI Model 1"));

        assert_eq!(marketplace.count_items(), 1);
    }
}