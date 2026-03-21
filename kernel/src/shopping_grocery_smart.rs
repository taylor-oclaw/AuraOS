extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut grocery_list = GroceryList::new();
    grocery_list.add_item(String::from("Milk"), 2);
    grocery_list.add_item(String::from("Bread"), 1);
    grocery_list.add_item(String::from("Eggs"), 12);


    if let Some(item) = grocery_list.find_item("Milk") {
    } else {
    }

    grocery_list.remove_item(String::from("Bread"));

    loop {}
}

pub struct GroceryItem {
    name: String,
    quantity: u32,
}

impl GroceryItem {
    pub fn new(name: String, quantity: u32) -> Self {
        GroceryItem { name, quantity }
    }
}

pub struct GroceryList {
    items: Vec<GroceryItem>,
}

impl GroceryList {
    pub fn new() -> Self {
        GroceryList { items: Vec::new() }
    }

    pub fn add_item(&mut self, name: String, quantity: u32) {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == name) {
            item.quantity += quantity;
        } else {
            self.items.push(GroceryItem::new(name, quantity));
        }
    }

    pub fn remove_item(&mut self, name: String) {
        self.items.retain(|item| item.name != name);
    }

    pub fn find_item(&self, name: &str) -> Option<&GroceryItem> {
        self.items.iter().find(|item| item.name == name)
    }

    pub fn total_items(&self) -> usize {
        self.items.len()
    }

    pub fn total_quantity(&self) -> u32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
}
