extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MiniAppRenderer {
    apps: Vec<String>,
    current_app_index: usize,
}

impl MiniAppRenderer {
    pub fn new() -> Self {
        MiniAppRenderer {
            apps: Vec::new(),
            current_app_index: 0,
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.apps.iter().position(|app| app == app_name) {
            self.apps.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_app(&self) -> Option<&String> {
        self.apps.get(self.current_app_index)
    }

    pub fn switch_to_next_app(&mut self) {
        if !self.apps.is_empty() {
            self.current_app_index = (self.current_app_index + 1) % self.apps.len();
        }
    }

    pub fn switch_to_previous_app(&mut self) {
        if !self.apps.is_empty() {
            self.current_app_index = if self.current_app_index == 0 {
                self.apps.len() - 1
            } else {
                self.current_app_index - 1
            };
        }
    }
}
