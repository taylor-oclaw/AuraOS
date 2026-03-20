extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn container_mkv_init() {
    // Initialization logic for the module
}

pub extern "C" fn container_mkv_exit() {
    // Cleanup logic for the module
}

pub struct ContainerMKV {
    items: Vec<String>,
}

impl ContainerMKV {
    pub fn new() -> Self {
        ContainerMKV {
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_item(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    pub fn get_all_items(&self) -> &Vec<String> {
        &self.items
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_mkv() {
        let mut container = ContainerMKV::new();
        assert_eq!(container.get_all_items().len(), 0);

        container.add_item(String::from("item1"));
        container.add_item(String::from("item2"));
        assert_eq!(container.get_all_items().len(), 2);
        assert_eq!(container.get_item(0), Some(&String::from("item1")));
        assert_eq!(container.get_item(1), Some(&String::from("item2")));

        let removed = container.remove_item(0);
        assert_eq!(removed, Some(String::from("item1")));
        assert_eq!(container.get_all_items().len(), 1);

        container.clear();
        assert_eq!(container.get_all_items().len(), 0);
    }
}
