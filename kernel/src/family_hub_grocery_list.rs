extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut grocery_list = GroceryList::new();
    grocery_list.add_item(String::from("Milk"));
    grocery_list.add_item(String::from("Bread"));
    grocery_list.add_item(String::from("Eggs"));

    if grocery_list.contains_item("Milk") {
    }

    let items = grocery_list.get_items();
    for item in items.iter() {
    }

    grocery_list.remove_item("Bread");
    let updated_items = grocery_list.get_items();
    for item in updated_items.iter() {
    }

    loop {}
}

pub struct GroceryList {
    items: Vec<String>,
}

impl GroceryList {
    pub fn new() -> Self {
        GroceryList { items: Vec::new() }
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
        self.items.contains(&item.to_string())
    }

    pub fn get_items(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn clear_list(&mut self) {
        self.items.clear();
    }
}
