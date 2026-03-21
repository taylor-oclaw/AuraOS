extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ProfileDeviceRoleFluid {
    device_id: String,
    role: String,
    capabilities: Vec<String>,
    status: String,
    last_updated: u64,
}

impl ProfileDeviceRoleFluid {
    pub fn new(device_id: &str, role: &str) -> Self {
        ProfileDeviceRoleFluid {
            device_id: String::from(device_id),
            role: String::from(role),
            capabilities: Vec::new(),
            status: String::from("active"),
            last_updated: 0,
        }
    }

    pub fn add_capability(&mut self, capability: &str) {
        self.capabilities.push(String::from(capability));
    }

    pub fn remove_capability(&mut self, capability: &str) -> bool {
        if let Some(index) = self.capabilities.iter().position(|c| c == capability) {
            self.capabilities.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_status(&mut self, status: &str) {
        self.status = String::from(status);
        self.last_updated = get_current_time(); // Assume this function exists to get current time in u64
    }

    pub fn get_device_id(&self) -> &str {
        &self.device_id
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    pub fn list_capabilities(&self) -> &[String] {
        &self.capabilities
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_last_updated(&self) -> u64 {
        self.last_updated
    }
}

fn get_current_time() -> u64 {
    // Placeholder for getting current time in u64, actual implementation depends on the kernel environment
    0
}
