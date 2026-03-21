extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut packing_list = TravelPackingList::new();
    packing_list.add_item(String::from("Passport"));
    packing_list.add_item(String::from("Travel Insurance"));
    packing_list.add_item(String::from("Laptop"));
    packing_list.add_item(String::from("Charger"));
    packing_list.add_item(String::from("Headphones"));

    if packing_list.contains_item("Passport") {
        println!("Passport is in the list.");
    }

    let items = packing_list.get_items();
    for item in items.iter() {
        println!("{}", item);
    }

    packing_list.remove_item("Charger");
    let remaining_items = packing_list.get_items();
    for item in remaining_items.iter() {
        println!("Remaining item: {}", item);
    }

    loop {}
}

pub struct TravelPackingList {
    items: Vec<String>,
}

impl TravelPackingList {
    pub fn new() -> Self {
        TravelPackingList { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) {
        if let Some(index) = self.items.iter().position(|x| x == item) {
            self.items.remove(index);
        }
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
