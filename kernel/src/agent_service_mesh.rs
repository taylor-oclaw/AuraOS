extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn agent_service_mesh_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn agent_service_mesh_exit() {
    // Cleanup logic for the module
}

pub struct AgentServiceMesh {
    services: Vec<String>,
    policies: Vec<String>,
}

impl AgentServiceMesh {
    pub fn new() -> Self {
        AgentServiceMesh {
            services: Vec::new(),
            policies: Vec::new(),
        }
    }

    pub fn add_service(&mut self, service_name: &str) {
        self.services.push(String::from(service_name));
    }

    pub fn remove_service(&mut self, service_name: &str) {
        if let Some(index) = self.services.iter().position(|s| s == service_name) {
            self.services.remove(index);
        }
    }

    pub fn list_services(&self) -> Vec<String> {
        self.services.clone()
    }

    pub fn add_policy(&mut self, policy: &str) {
        self.policies.push(String::from(policy));
    }

    pub fn remove_policy(&mut self, policy: &str) {
        if let Some(index) = self.policies.iter().position(|p| p == policy) {
            self.policies.remove(index);
        }
    }

    pub fn list_policies(&self) -> Vec<String> {
        self.policies.clone()
    }
}
