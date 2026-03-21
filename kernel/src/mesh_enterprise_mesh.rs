extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_enterprise_mesh_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_enterprise_mesh_exit() {
    // Cleanup logic for the module
}

pub struct MeshEnterpriseMesh {
    nodes: Vec<String>,
    connections: Vec<(String, String)>,
}

impl MeshEnterpriseMesh {
    pub fn new() -> Self {
        MeshEnterpriseMesh {
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
            // Remove all connections involving this node
            self.connections.retain(|&(ref a, ref b)| a != node_name && b != node_name);
        }
    }

    pub fn connect_nodes(&mut self, node1: &str, node2: &str) {
        if self.nodes.contains(&node1.to_string()) && self.nodes.contains(&node2.to_string()) {
            self.connections.push((node1.to_string(), node2.to_string()));
        }
    }

    pub fn disconnect_nodes(&mut self, node1: &str, node2: &str) {
        self.connections.retain(|&(ref a, ref b)| !(a == node1 && b == node2));
    }

    pub fn list_connections(&self) -> Vec<(String, String)> {
        self.connections.clone()
    }
}
