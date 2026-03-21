extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut engine = ActionItemEngine::new();
    engine.add_item(String::from("Complete Rust module"));
    engine.add_item(String::from("Implement new features"));
    engine.complete_item(0);
    engine.remove_item(1);
    let items = engine.get_items();
    for item in items {
        println!("{}", item);
    }
}

pub struct ActionItemEngine {
    items: Vec<String>,
    completed: Vec<bool>,
}

impl ActionItemEngine {
    pub fn new() -> Self {
        ActionItemEngine {
            items: Vec::new(),
            completed: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
        self.completed.push(false);
    }

    pub fn complete_item(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.items.len() {
            self.completed[index] = true;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn remove_item(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.items.len() {
            self.items.remove(index);
            self.completed.remove(index);
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn get_items(&self) -> Vec<String> {
        let mut result = Vec::new();
        for (i, item) in self.items.iter().enumerate() {
            if self.completed[i] {
                result.push(format!("{} [Completed]", item));
            } else {
                result.push(item.clone());
            }
        }
        result
    }

    pub fn get_completed_count(&self) -> usize {
        self.completed.iter().filter(|&&c| c).count()
    }
}
