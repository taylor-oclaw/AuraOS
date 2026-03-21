extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ActionItem {
    id: u32,
    description: String,
    completed: bool,
}

impl ActionItem {
    pub fn new(id: u32, description: &str) -> Self {
        ActionItem {
            id,
            description: String::from(description),
            completed: false,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn update_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }
}

pub struct ActionItemTracker {
    items: Vec<ActionItem>,
}

impl ActionItemTracker {
    pub fn new() -> Self {
        ActionItemTracker {
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: ActionItem) {
        self.items.push(item);
    }

    pub fn get_items(&self) -> &Vec<ActionItem> {
        &self.items
    }

    pub fn mark_item_completed(&mut self, id: u32) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.mark_completed();
        }
    }

    pub fn update_item_description(&mut self, id: u32, new_description: &str) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.update_description(new_description);
        }
    }

    pub fn remove_item(&mut self, id: u32) {
        self.items.retain(|item| item.id != id);
    }
}
