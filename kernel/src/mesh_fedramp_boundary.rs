extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_fedramp_boundary_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_fedramp_boundary_exit() {
    // Cleanup logic for the module
}

pub struct MeshFedRampBoundary {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

impl MeshFedRampBoundary {
    pub fn new() -> Self {
        MeshFedRampBoundary {
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
        if let Some(i) = index {
            self.nodes.remove(i);
            self.edges.retain(|&(ref a, ref b)| a != node_name && b != node_name);
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if self.nodes.contains(&from.to_string()) && self.nodes.contains(&to.to_string()) {
            self.edges.push((from.to_string(), to.to_string()));
        }
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) {
        self.edges.retain(|&(ref a, ref b)| !(a == from && b == to));
    }

    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }
}
