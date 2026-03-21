extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod kernel_module {
    use super::*;

    pub struct MarketplaceDemoMode {
        items: Vec<String>,
        active_item_index: usize,
    }

    impl MarketplaceDemoMode {
        pub fn new() -> Self {
            MarketplaceDemoMode {
                items: Vec::new(),
                active_item_index: 0,
            }
        }

        pub fn add_item(&mut self, item_name: &str) {
            self.items.push(String::from(item_name));
            if self.items.len() == 1 {
                self.active_item_index = 0;
            }
        }

        pub fn remove_item(&mut self, index: usize) -> Option<String> {
            if index < self.items.len() {
                let removed_item = self.items.remove(index);
                if self.active_item_index >= self.items.len() && !self.items.is_empty() {
                    self.active_item_index = self.items.len() - 1;
                }
                Some(removed_item)
            } else {
                None
            }
        }

        pub fn get_active_item(&self) -> Option<&String> {
            if self.items.is_empty() {
                None
            } else {
                Some(&self.items[self.active_item_index])
            }
        }

        pub fn set_active_item(&mut self, index: usize) -> bool {
            if index < self.items.len() {
                self.active_item_index = index;
                true
            } else {
                false
            }
        }

        pub fn list_items(&self) -> Vec<&String> {
            self.items.iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::kernel_module::MarketplaceDemoMode;

    #[test]
    fn test_marketplace_demo_mode() {
        let mut marketplace = MarketplaceDemoMode::new();
        assert_eq!(marketplace.get_active_item(), None);

        marketplace.add_item("AI Assistant");
        assert_eq!(marketplace.get_active_item(), Some(&String::from("AI Assistant")));

        marketplace.add_item("Smartphone");
        assert_eq!(marketplace.list_items(), vec![&String::from("AI Assistant"), &String::from("Smartphone")]);
        assert_eq!(marketplace.get_active_item(), Some(&String::from("AI Assistant")));

        marketplace.set_active_item(1);
        assert_eq!(marketplace.get_active_item(), Some(&String::from("Smartphone")));

        let removed_item = marketplace.remove_item(0);
        assert_eq!(removed_item, Some(String::from("AI Assistant")));
        assert_eq!(marketplace.list_items(), vec![&String::from("Smartphone")]);
        assert_eq!(marketplace.get_active_item(), Some(&String::from("Smartphone")));

        let removed_item = marketplace.remove_item(0);
        assert_eq!(removed_item, Some(String::from("Smartphone")));
        assert_eq!(marketplace.list_items(), Vec::<&String>::new());
        assert_eq!(marketplace.get_active_item(), None);

        assert!(!marketplace.set_active_item(1));
    }
}
