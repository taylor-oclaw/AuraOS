extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MenuSystem {
    items: Vec<String>,
    selected_index: usize,
}

impl MenuSystem {
    pub fn new() -> Self {
        MenuSystem {
            items: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
        if self.selected_index >= self.items.len() {
            self.selected_index = self.items.len() - 1;
        }
    }

    pub fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            let removed_item = self.items.remove(index);
            if self.selected_index >= self.items.len() {
                self.selected_index = self.items.len().saturating_sub(1);
            }
            Some(removed_item)
        } else {
            None
        }
    }

    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    pub fn select_previous(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.items.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn get_selected_item(&self) -> Option<&String> {
        self.items.get(self.selected_index)
    }
}
