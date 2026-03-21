extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshSpotCompute {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl MeshSpotCompute {
    pub fn new() -> Self {
        MeshSpotCompute {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: String) {
        self.nodes.push(node_name);
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.edges.push((from, to));
            true
        } else {
            false
        }
    }

    pub fn get_nodes(&self) -> &Vec<String> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }

    pub fn is_connected(&self, from: usize, to: usize) -> bool {
        self.edges.iter().any(|&(a, b)| (a == from && b == to) || (a == to && b == from))
    }
}
