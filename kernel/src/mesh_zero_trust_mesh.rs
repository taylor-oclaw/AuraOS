extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshZeroTrustMesh {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

impl MeshZeroTrustMesh {
    pub fn new() -> Self {
        MeshZeroTrustMesh {
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
            self.edges.retain(|&(ref from, ref to)| from != node_name && to != node_name);
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        if self.nodes.contains(&from.to_string()) && self.nodes.contains(&to.to_string()) {
            self.edges.push((from.to_string(), to.to_string()));
        }
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) {
        self.edges.retain(|&(ref f, ref t)| f != from || t != to);
    }

    pub fn get_neighbors(&self, node_name: &str) -> Vec<String> {
        let mut neighbors = Vec::new();
        for &(ref from, ref to) in &self.edges {
            if from == node_name {
                neighbors.push(to.clone());
            } else if to == node_name {
                neighbors.push(from.clone());
            }
        }
        neighbors
    }
}
