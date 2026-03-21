extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ActionItem {
    description: String,
    priority: u8,
}

impl ActionItem {
    pub fn new(description: &str, priority: u8) -> Self {
        ActionItem {
            description: String::from(description),
            priority,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    pub fn get_priority(&self) -> u8 {
        self.priority
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }
}

#[derive(Debug)]
pub struct ActionItemPriority {
    items: Vec<ActionItem>,
}

impl ActionItemPriority {
    pub fn new() -> Self {
        ActionItemPriority { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: ActionItem) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<ActionItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_items(&self) -> &Vec<ActionItem> {
        &self.items
    }

    pub fn sort_by_priority(&mut self) {
        self.items.sort_by_key(|item| item.priority);
    }
}
