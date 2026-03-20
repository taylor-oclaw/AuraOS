extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut attention = LinearAttentionV2::new();
    attention.add_item("item1".into(), 0.5);
    attention.add_item("item2".into(), 0.3);
    attention.add_item("item3".into(), 0.2);

    let top_item = attention.get_top_item().unwrap();
    println!("Top item: {}", top_item);

    let total_weight = attention.total_weight();
    println!("Total weight: {}", total_weight);

    attention.update_weight("item1", 0.7);
    let updated_top_item = attention.get_top_item().unwrap();
    println!("Updated top item: {}", updated_top_item);

    attention.remove_item("item2");
    let remaining_items = attention.list_items();
    println!("Remaining items: {:?}", remaining_items);

    loop {}
}

pub struct LinearAttentionV2 {
    items: Vec<(String, f32)>,
}

impl LinearAttentionV2 {
    pub fn new() -> Self {
        LinearAttentionV2 { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: String, weight: f32) {
        self.items.push((item, weight));
    }

    pub fn get_top_item(&self) -> Option<&String> {
        self.items.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|i| &i.0)
    }

    pub fn total_weight(&self) -> f32 {
        self.items.iter().map(|(_, weight)| *weight).sum()
    }

    pub fn update_weight(&mut self, item: &str, new_weight: f32) {
        if let Some(index) = self.items.iter().position(|(i, _)| i == item) {
            self.items[index].1 = new_weight;
        }
    }

    pub fn remove_item(&mut self, item: &str) {
        self.items.retain(|(i, _)| i != item);
    }

    pub fn list_items(&self) -> Vec<&String> {
        self.items.iter().map(|(item, _)| item).collect()
    }
}
