extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshActiveActive {
    nodes: Vec<String>,
    connections: Vec<(usize, usize)>,
}

impl MeshActiveActive {
    pub fn new() -> Self {
        MeshActiveActive {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: String) {
        self.nodes.push(node_name);
    }

    pub fn remove_node(&mut self, node_index: usize) -> Option<String> {
        if node_index < self.nodes.len() {
            let removed_node = self.nodes.remove(node_index);
            self.connections.retain(|&(a, b)| a != node_index && b != node_index);
            Some(removed_node)
        } else {
            None
        }
    }

    pub fn connect_nodes(&mut self, node_a: usize, node_b: usize) -> bool {
        if node_a < self.nodes.len() && node_b < self.nodes.len() && node_a != node_b {
            self.connections.push((node_a, node_b));
            true
        } else {
            false
        }
    }

    pub fn disconnect_nodes(&mut self, node_a: usize, node_b: usize) -> bool {
        let index = self.connections.iter().position(|&(a, b)| a == node_a && b == node_b);
        if let Some(pos) = index {
            self.connections.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn list_connections(&self) -> Vec<(String, String)> {
        self.connections.iter()
            .filter_map(|&(a, b)| {
                if a < self.nodes.len() && b < self.nodes.len() {
                    Some((self.nodes[a].clone(), self.nodes[b].clone()))
                } else {
                    None
                }
            }
            .collect()
    }
}
