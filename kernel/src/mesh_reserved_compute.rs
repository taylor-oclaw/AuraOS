extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshReservedCompute {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl MeshReservedCompute {
    pub fn new() -> Self {
        MeshReservedCompute {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        let node = String::from(node_name);
        self.nodes.push(node);
    }

    pub fn remove_node(&mut self, node_index: usize) -> Option<String> {
        if node_index < self.nodes.len() {
            Some(self.nodes.remove(node_index))
        } else {
            None
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.edges.push((from, to));
        }
    }

    pub fn remove_edge(&mut self, edge_index: usize) -> Option<(usize, usize)> {
        if edge_index < self.edges.len() {
            Some(self.edges.remove(edge_index))
        } else {
            None
        }
    }

    pub fn get_neighbors(&self, node_index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        for &(from, to) in &self.edges {
            if from == node_index {
                neighbors.push(to);
            } else if to == node_index {
                neighbors.push(from);
            }
        }
        neighbors
    }
}
