extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RelationshipGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>, // (from_index, to_index)
}

impl RelationshipGraph {
    pub fn new() -> Self {
        RelationshipGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: String) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node_name);
        index
    }

    pub fn add_edge(&mut self, from_index: usize, to_index: usize) {
        if from_index < self.nodes.len() && to_index < self.nodes.len() {
            self.edges.push((from_index, to_index));
        }
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn has_edge(&self, from_index: usize, to_index: usize) -> bool {
        self.edges.iter().any(|&(f, t)| f == from_index && t == to_index)
    }
}
