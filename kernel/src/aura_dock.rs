extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

pub struct AuraDock {
    name: String,
    items: Vec<String>,
}

impl AuraDock {
    pub fn new(name: &str) -> Self {
        AuraDock {
            name: String::from(name),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.items.iter().position(|x| x == item) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_items(&self) -> &[String] {
        &self.items
    }

    pub fn has_item(&self, item: &str) -> bool {
        self.items.contains(&String::from(item))
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_dock() {
        let mut dock = AuraDock::new("Test Dock");
        assert_eq!(dock.get_items().len(), 0);

        dock.add_item("Item1");
        dock.add_item("Item2");
        assert_eq!(dock.get_items().len(), 2);
        assert!(dock.has_item("Item1"));
        assert!(!dock.has_item("Item3"));

        assert!(dock.remove_item("Item1"));
        assert!(!dock.remove_item("Item1"));
        assert_eq!(dock.get_items().len(), 1);

        dock.clear_items();
        assert_eq!(dock.get_items().len(), 0);
    }
}
