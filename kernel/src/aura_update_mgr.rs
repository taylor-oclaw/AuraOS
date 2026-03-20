extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraUpdateMgr {
    updates: Vec<String>,
    current_version: String,
}

impl AuraUpdateMgr {
    pub fn new(initial_version: &str) -> Self {
        AuraUpdateMgr {
            updates: Vec::new(),
            current_version: initial_version.to_string(),
        }
    }

    pub fn add_update(&mut self, update_description: &str) {
        self.updates.push(update_description.to_string());
    }

    pub fn get_current_version(&self) -> &str {
        &self.current_version
    }

    pub fn list_updates(&self) -> &[String] {
        &self.updates
    }

    pub fn apply_update(&mut self, update_index: usize) -> Result<(), &'static str> {
        if let Some(update) = self.updates.get(update_index) {
            // Simulate applying the update
            self.current_version = update.clone();
            Ok(())
        } else {
            Err("Update index out of bounds")
        }
    }

    pub fn clear_updates(&mut self) {
        self.updates.clear();
    }
}
