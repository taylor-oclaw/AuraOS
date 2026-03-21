extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_ediscovery_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_ediscovery_exit() {
    // Cleanup logic for the module
}

pub struct MeshDiscovery {
    nodes: Vec<String>,
    services: Vec<String>,
}

impl MeshDiscovery {
    pub fn new() -> Self {
        MeshDiscovery {
            nodes: Vec::new(),
            services: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: &str) {
        if !self.nodes.contains(&node_id.to_string()) {
            self.nodes.push(node_id.to_string());
        }
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.retain(|n| n != node_id);
    }

    pub fn add_service(&mut self, service_name: &str) {
        if !self.services.contains(&service_name.to_string()) {
            self.services.push(service_name.to_string());
        }
    }

    pub fn remove_service(&mut self, service_name: &str) {
        self.services.retain(|s| s != service_name);
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn list_services(&self) -> Vec<String> {
        self.services.clone()
    }
}
