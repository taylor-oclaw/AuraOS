extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_personal_discovery_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_personal_discovery_exit() {
    // Cleanup logic for the module
}

pub struct MeshPersonalDiscovery {
    nodes: Vec<String>,
    discovered_nodes: usize,
}

impl MeshPersonalDiscovery {
    pub fn new() -> Self {
        MeshPersonalDiscovery {
            nodes: Vec::new(),
            discovered_nodes: 0,
        }
    }

    pub fn add_node(&mut self, node_id: &str) {
        if !self.nodes.contains(&node_id.to_string()) {
            self.nodes.push(node_id.to_string());
            self.discovered_nodes += 1;
        }
    }

    pub fn remove_node(&mut self, node_id: &str) -> bool {
        let index = self.nodes.iter().position(|n| n == node_id);
        if let Some(i) = index {
            self.nodes.remove(i);
            self.discovered_nodes -= 1;
            true
        } else {
            false
        }
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn is_node_discovered(&self, node_id: &str) -> bool {
        self.nodes.contains(&node_id.to_string())
    }
}
