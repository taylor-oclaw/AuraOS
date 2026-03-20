extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialization logic for the kernel module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup logic for the kernel module
}

pub struct AuraAgentMarketplaceV2 {
    agents: Vec<String>,
    services: Vec<String>,
}

impl AuraAgentMarketplaceV2 {
    pub fn new() -> Self {
        AuraAgentMarketplaceV2 {
            agents: Vec::new(),
            services: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: &str) {
        self.agents.push(String::from(agent_name));
    }

    pub fn remove_agent(&mut self, agent_name: &str) -> bool {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            self.agents.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agents.clone()
    }

    pub fn add_service(&mut self, service_name: &str) {
        self.services.push(String::from(service_name));
    }

    pub fn remove_service(&mut self, service_name: &str) -> bool {
        if let Some(index) = self.services.iter().position(|s| s == service_name) {
            self.services.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_services(&self) -> Vec<String> {
        self.services.clone()
    }
}
