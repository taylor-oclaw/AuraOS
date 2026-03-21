extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingAgenda {
    title: String,
    items: Vec<String>,
}

impl MeetingAgenda {
    pub fn new(title: &str) -> Self {
        MeetingAgenda {
            title: String::from(title),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn list_items(&self) -> &[String] {
        &self.items
    }

    pub fn clear_agenda(&mut self) {
        self.items.clear();
    }
}
