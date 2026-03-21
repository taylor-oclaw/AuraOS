extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MeshWorkloadIdentity {
    workload_id: String,
    identity_token: String,
    permissions: Vec<String>,
    last_updated: u64, // Unix timestamp
    is_active: bool,
}

impl MeshWorkloadIdentity {
    pub fn new(workload_id: &str, identity_token: &str) -> Self {
        MeshWorkloadIdentity {
            workload_id: workload_id.to_string(),
            identity_token: identity_token.to_string(),
            permissions: Vec::new(),
            last_updated: 0,
            is_active: true,
        }
    }

    pub fn add_permission(&mut self, permission: &str) {
        if !self.permissions.contains(&permission.to_string()) {
            self.permissions.push(permission.to_string());
        }
    }

    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.retain(|p| p != permission);
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    pub fn update_token(&mut self, new_token: &str) {
        self.identity_token = new_token.to_string();
        self.last_updated = current_time(); // Assume a function to get current time
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

fn current_time() -> u64 {
    // Placeholder for getting the current time in Unix timestamp format
    0
}
