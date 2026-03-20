extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct TabControl {
    tabs: Vec<String>,
    active_index: usize,
}

impl TabControl {
    pub fn new() -> Self {
        TabControl {
            tabs: Vec::new(),
            active_index: 0,
        }
    }

    pub fn add_tab(&mut self, title: &str) {
        let tab_title = String::from(title);
        self.tabs.push(tab_title);
        if self.active_index >= self.tabs.len() {
            self.active_index = self.tabs.len() - 1;
        }
    }

    pub fn remove_tab(&mut self, index: usize) -> Option<String> {
        if index < self.tabs.len() {
            let removed_tab = self.tabs.remove(index);
            if self.active_index >= self.tabs.len() {
                self.active_index = self.tabs.len().saturating_sub(1);
            }
            Some(removed_tab)
        } else {
            None
        }
    }

    pub fn activate_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_index = index;
            true
        } else {
            false
        }
    }

    pub fn get_active_tab_title(&self) -> Option<&str> {
        self.tabs.get(self.active_index).map(|s| s.as_str())
    }

    pub fn list_tabs(&self) -> Vec<&str> {
        self.tabs.iter().map(|s| s.as_str()).collect()
    }
}
