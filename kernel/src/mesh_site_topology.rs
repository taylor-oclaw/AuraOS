extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct MeshSiteTopology {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl MeshSiteTopology {
    pub fn new() -> Self {
        MeshSiteTopology {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        let name = String::from(node_name);
        self.nodes.push(name);
    }

    pub fn remove_node(&mut self, node_index: usize) -> Option<String> {
        if node_index < self.nodes.len() {
            let removed_node = self.nodes.remove(node_index);
            self.edges.retain(|&(a, b)| a != node_index && b != node_index);
            Some(removed_node)
        } else {
            None
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        if from < self.nodes.len() && to < self.nodes.len() && from != to {
            self.edges.push((from, to));
            true
        } else {
            false
        }
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> Option<(usize, usize)> {
        let index = self.edges.iter().position(|&(a, b)| a == from && b == to);
        if let Some(idx) = index {
            Some(self.edges.remove(idx))
        } else {
            None
        }
    }

    pub fn get_neighbors(&self, node_index: usize) -> Vec<usize> {
        self.edges.iter()
            .filter(|&&(a, b)| a == node_index || b == node_index)
            .map(|&(a, b)| if a == node_index { b } else { a })
            .collect()
    }
}
