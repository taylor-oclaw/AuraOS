extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshDataFlowMap {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl MeshDataFlowMap {
    pub fn new() -> Self {
        MeshDataFlowMap {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) -> usize {
        let index = self.nodes.len();
        self.nodes.push(String::from(node_name));
        index
    }

    pub fn add_edge(&mut self, from_index: usize, to_index: usize) {
        if from_index < self.nodes.len() && to_index < self.nodes.len() {
            self.edges.push((from_index, to_index));
        }
    }

    pub fn get_nodes(&self) -> &Vec<String> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }

    pub fn find_node_by_name(&self, node_name: &str) -> Option<usize> {
        self.nodes.iter().position(|n| n == node_name)
    }
}
