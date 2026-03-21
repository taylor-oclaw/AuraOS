extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_sox_boundary_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_sox_boundary_exit() {
    // Cleanup logic for the module
}

pub struct MeshSoxBoundary {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

impl MeshSoxBoundary {
    pub fn new() -> Self {
        MeshSoxBoundary {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: String) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    pub fn remove_node(&mut self, node: &str) {
        let index = self.nodes.iter().position(|n| n == node);
        if let Some(i) = index {
            self.nodes.remove(i);
            self.edges.retain(|&(ref a, ref b)| a != node && b != node);
        }
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        if self.nodes.contains(&from) && self.nodes.contains(&to) {
            self.edges.push((from, to));
        }
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) {
        self.edges.retain(|&(ref a, ref b)| a != from || b != to);
    }

    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }
}
