extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAppStore {
    apps: Vec<String>,
}

impl AuraAppStore {
    pub fn new() -> Self {
        AuraAppStore { apps: Vec::new() }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) {
        if let Some(index) = self.apps.iter().position(|app| *app == app_name) {
            self.apps.remove(index);
        }
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn has_app(&self, app_name: &str) -> bool {
        self.apps.contains(&String::from(app_name))
    }

    pub fn count_apps(&self) -> usize {
        self.apps.len()
    }
}
