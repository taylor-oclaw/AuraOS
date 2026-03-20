extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct TrustReputationGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize, i32)>, // (from_node_index, to_node_index, reputation_score)
}

impl TrustReputationGraph {
    pub fn new() -> Self {
        TrustReputationGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        let node = String::from(node_name);
        self.nodes.push(node);
    }

    pub fn get_node_index(&self, node_name: &str) -> Option<usize> {
        self.nodes.iter().position(|n| n == node_name)
    }

    pub fn add_edge(&mut self, from_node: &str, to_node: &str, reputation_score: i32) {
        if let (Some(from_index), Some(to_index)) = (self.get_node_index(from_node), self.get_node_index(to_node)) {
            self.edges.push((from_index, to_index, reputation_score));
        }
    }

    pub fn get_reputation(&self, from_node: &str, to_node: &str) -> Option<i32> {
        if let (Some(from_index), Some(to_index)) = (self.get_node_index(from_node), self.get_node_index(to_node)) {
            for &(f, t, score) in &self.edges {
                if f == from_index && t == to_index {
                    return Some(score);
                }
            }
        }
        None
    }

    pub fn update_reputation(&mut self, from_node: &str, to_node: &str, new_score: i32) {
        if let (Some(from_index), Some(to_index)) = (self.get_node_index(from_node), self.get_node_index(to_node)) {
            for edge in &mut self.edges {
                if *edge == (from_index, to_index, edge.2) {
                    edge.2 = new_score;
                    break;
                }
            }
        }
    }
}
