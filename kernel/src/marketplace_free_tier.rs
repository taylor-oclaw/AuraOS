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

    pub fn add_item(&mut self, item: String) -> bool {
        if self.items.len() < self.max_items {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_items(&self) -> &[String] {
        &self.items
    }

    pub fn is_full(&self) -> bool {
        self.items.len() == self.max_items
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace_free_tier() {
        let mut marketplace = MarketplaceFreeTier::new(3);

        assert_eq!(marketplace.add_item(String::from("Item1")), true);
        assert_eq!(marketplace.add_item(String::from("Item2")), true);
        assert_eq!(marketplace.add_item(String::from("Item3")), true);
        assert_eq!(marketplace.add_item(String::from("Item4")), false);

        assert_eq!(marketplace.get_items(), vec!["Item1", "Item2", "Item3"]);

        assert_eq!(marketplace.remove_item(1), Some(String::from("Item2")));
        assert_eq!(marketplace.get_items(), vec!["Item1", "Item3"]);

        assert_eq!(marketplace.is_full(), false);
        marketplace.add_item(String::from("Item4"));
        assert_eq!(marketplace.is_full(), true);

        marketplace.clear();
        assert_eq!(marketplace.get_items().len(), 0);
    }
}
