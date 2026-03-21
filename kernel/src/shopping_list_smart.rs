extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    let mut list = ShoppingListSmart::new();
    list.add_item(String::from("Apples"));
    list.add_item(String::from("Bananas"));
    list.add_item(String::from("Carrots"));

    if list.contains_item("Bananas") {
        list.remove_item("Bananas");
    }

    list.print_list();

    0
}

pub struct ShoppingListSmart {
    items: Vec<String>,
}

impl ShoppingListSmart {
    pub fn new() -> Self {
        ShoppingListSmart { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: String) {
        if !self.items.contains(&item) {
            self.items.push(item);
        }
    }

    pub fn remove_item(&mut self, item: &str) {
        self.items.retain(|i| i != item);
    }

    pub fn contains_item(&self, item: &str) -> bool {
        self.items.iter().any(|i| i == item)
    }

    pub fn get_items(&self) -> &[String] {
        &self.items
    }

    pub fn print_list(&self) {
        for item in &self.items {
            // Simulate printing to console or log
        }
    }
}
