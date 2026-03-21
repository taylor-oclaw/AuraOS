extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_industry_defense_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_industry_defense_exit() {
    // Cleanup logic for the module
}

pub struct MeshIndustryDefense {
    nodes: Vec<String>,
    connections: Vec<(String, String)>,
}

impl MeshIndustryDefense {
    pub fn new() -> Self {
        MeshIndustryDefense {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        if !self.nodes.contains(&node_name.to_string()) {
            self.nodes.push(node_name.to_string());
        }
    }

    pub fn remove_node(&mut self, node_name: &str) {
        let index = self.nodes.iter().position(|n| n == node_name);
        if let Some(i) = index {
            self.nodes.remove(i);
            self.connections.retain(|&(ref a, ref b)| a != node_name && b != node_name);
        }
    }

    pub fn connect_nodes(&mut self, from: &str, to: &str) {
        if self.nodes.contains(&from.to_string()) && self.nodes.contains(&to.to_string()) {
            self.connections.push((from.to_string(), to.to_string()));
        }
    }

    pub fn disconnect_nodes(&mut self, from: &str, to: &str) {
        self.connections.retain(|&(ref a, ref b)| !(a == from && b == to));
    }

    pub fn list_connections(&self) -> Vec<(String, String)> {
        self.connections.clone()
    }
}
