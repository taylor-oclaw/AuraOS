extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct MiniAppLayoutEngine {
    apps: Vec<String>,
    layout: String,
}

impl MiniAppLayoutEngine {
    pub fn new() -> Self {
        MiniAppLayoutEngine {
            apps: Vec::new(),
            layout: String::from("default"),
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.apps.iter().position(|x| x == app_name) {
            self.apps.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn set_layout(&mut self, layout: &str) {
        self.layout = String::from(layout);
    }

    pub fn get_layout(&self) -> String {
        self.layout.clone()
    }
}
