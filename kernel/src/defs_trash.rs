extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct TrashCan {
    items: Vec<String>,
}

impl TrashCan {
    pub fn new() -> Self {
        TrashCan { items: Vec::new() }
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

    pub fn list_items(&self) -> Vec<&str> {
        self.items.iter().map(|item| item.as_str()).collect()
    }

    pub fn count_items(&self) -> usize {
        self.items.len()
    }

    pub fn clear_trash(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trash_can() {
        let mut trash = TrashCan::new();
        assert_eq!(trash.count_items(), 0);

        trash.add_item(String::from("Paper"));
        trash.add_item(String::from("Plastic"));
        assert_eq!(trash.count_items(), 2);

        let items = trash.list_items();
        assert_eq!(items, vec!["Paper", "Plastic"]);

        let removed = trash.remove_item(0);
        assert_eq!(removed, Some(String::from("Paper")));
        assert_eq!(trash.count_items(), 1);

        trash.clear_trash();
        assert_eq!(trash.count_items(), 0);
    }
}
