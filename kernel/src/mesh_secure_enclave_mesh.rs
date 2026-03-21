extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_secure_enclave_mesh_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_secure_enclave_mesh_exit() {
    // Cleanup logic for the module
}

pub struct MeshSecureEnclaveMesh {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

impl MeshSecureEnclaveMesh {
    pub fn new() -> Self {
        MeshSecureEnclaveMesh {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        if !self.nodes.contains(&node_name.to_string()) {
            self.nodes.push(node_name.to_string());
        }
    }

    pub fn remove_node(&mut self, node_name: &str) {
        let index = self.nodes.iter().position(|n| n == node_name);
        if let Some(idx) = index {
            self.nodes.remove(idx);
            self.edges.retain(|&(ref a, ref b)| a != node_name && b != node_name);
        }
    }

    pub fn add_edge(&mut self, from_node: &str, to_node: &str) {
        if self.nodes.contains(&from_node.to_string()) && self.nodes.contains(&to_node.to_string()) {
            self.edges.push((from_node.to_string(), to_node.to_string()));
        }
    }

    pub fn remove_edge(&mut self, from_node: &str, to_node: &str) {
        self.edges.retain(|&(ref a, ref b)| a != from_node || b != to_node);
    }

    pub fn get_neighbors(&self, node_name: &str) -> Vec<String> {
        let mut neighbors = Vec::new();
        for &(ref from, ref to) in &self.edges {
            if from == node_name {
                neighbors.push(to.clone());
            } else if to == node_name {
                neighbors.push(from.clone());
            }
        }
        neighbors
    }
}
