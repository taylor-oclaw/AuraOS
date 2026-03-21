extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_right_to_delete_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_right_to_delete_exit() {
    // Cleanup logic for the module
}

pub struct MeshRightToDelete {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

impl MeshRightToDelete {
    pub fn new() -> Self {
        MeshRightToDelete {
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
            self.edges.retain(|&(ref from, ref to)| from != node && to != node);
        }
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        if self.nodes.contains(&from) && self.nodes.contains(&to) {
            self.edges.push((from, to));
        }
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) {
        self.edges.retain(|&(ref f, ref t)| f != from || t != to);
    }

    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn get_edges(&self) -> Vec<(String, String)> {
        self.edges.clone()
    }
}
