extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AIModelAccessControl {
    models: Vec<String>,
    permissions: Vec<u8>, // 0 - no access, 1 - read only, 2 - full access
}

impl AIModelAccessControl {
    pub fn new() -> Self {
        AIModelAccessControl {
            models: Vec::new(),
            permissions: Vec::new(),
        }
    }

    pub fn add_model(&mut self, model_name: &str, permission: u8) {
        if !self.models.contains(&String::from(model_name)) {
            self.models.push(String::from(model_name));
            self.permissions.push(permission);
        }
    }

    pub fn remove_model(&mut self, model_name: &str) {
        if let Some(index) = self.models.iter().position(|m| m == model_name) {
            self.models.remove(index);
            self.permissions.remove(index);
        }
    }

    pub fn get_permission(&self, model_name: &str) -> Option<u8> {
        self.models.iter().position(|m| m == model_name).map(|index| self.permissions[index])
    }

    pub fn update_permission(&mut self, model_name: &str, new_permission: u8) {
        if let Some(index) = self.models.iter().position(|m| m == model_name) {
            self.permissions[index] = new_permission;
        }
    }

    pub fn list_models(&self) -> Vec<String> {
        self.models.clone()
    }
}
