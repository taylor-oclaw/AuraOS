extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_trusted_execution_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_trusted_execution_exit() {
    // Cleanup logic for the module
}

pub struct MeshTrustedExecution {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

impl MeshTrustedExecution {
    pub fn new() -> Self {
        MeshTrustedExecution {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: &str) {
        if !self.nodes.contains(&node.to_string()) {
            self.nodes.push(node.to_string());
        }
    }

    pub fn remove_node(&mut self, node: &str) {
        let index = self.nodes.iter().position(|n| n == node);
        if let Some(i) = index {
            self.nodes.remove(i);
            self.edges.retain(|&(ref a, ref b)| a != node && b != node);
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if self.nodes.contains(&from.to_string()) && self.nodes.contains(&to.to_string()) {
            self.edges.push((from.to_string(), to.to_string()));
        }
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) {
        self.edges.retain(|&(ref a, ref b)| a != from || b != to);
    }

    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }
}
