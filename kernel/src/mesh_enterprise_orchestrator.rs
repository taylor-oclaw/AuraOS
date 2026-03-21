extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct MeshEnterpriseOrchestrator {
    nodes: Vec<String>,
    services: Vec<String>,
    policies: Vec<String>,
    configurations: Vec<String>,
    logs: Vec<String>,
}

impl MeshEnterpriseOrchestrator {
    pub fn new() -> Self {
        MeshEnterpriseOrchestrator {
            nodes: Vec::new(),
            services: Vec::new(),
            policies: Vec::new(),
            configurations: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        self.nodes.push(String::from(node_name));
    }

    pub fn remove_node(&mut self, node_name: &str) -> bool {
        if let Some(index) = self.nodes.iter().position(|n| n == node_name) {
            self.nodes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
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

    pub fn add_policy(&mut self, policy: &str) {
        self.policies.push(String::from(policy));
    }

    pub fn remove_policy(&mut self, policy: &str) -> bool {
        if let Some(index) = self.policies.iter().position(|p| p == policy) {
            self.policies.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_policies(&self) -> Vec<String> {
        self.policies.clone()
    }

    pub fn add_configuration(&mut self, config: &str) {
        self.configurations.push(String::from(config));
    }

    pub fn remove_configuration(&mut self, config: &str) -> bool {
        if let Some(index) = self.configurations.iter().position(|c| c == config) {
            self.configurations.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_configurations(&self) -> Vec<String> {
        self.configurations.clone()
    }

    pub fn log_event(&mut self, event: &str) {
        self.logs.push(String::from(event));
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}
