extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshOrchestrator {
    nodes: Vec<String>,
    connections: Vec<(usize, usize)>,
}

impl MeshOrchestrator {
    pub fn new() -> Self {
        MeshOrchestrator {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: String) -> bool {
        if !self.nodes.contains(&node_name) {
            self.nodes.push(node_name);
            true
        } else {
            false
        }
    }

    pub fn remove_node(&mut self, node_name: &str) -> bool {
        if let Some(index) = self.nodes.iter().position(|n| n == node_name) {
            self.nodes.remove(index);
            self.connections.retain(|&(a, b)| a != index && b != index);
            for (i, conn) in self.connections.iter_mut().enumerate() {
                if conn.0 > index {
                    self.connections[i].0 -= 1;
                }
                if conn.1 > index {
                    self.connections[i].1 -= 1;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn connect_nodes(&mut self, node_name_a: &str, node_name_b: &str) -> bool {
        if let Some(index_a) = self.nodes.iter().position(|n| n == node_name_a) {
            if let Some(index_b) = self.nodes.iter().position(|n| n == node_name_b) {
                if index_a != index_b && !self.connections.contains(&(index_a, index_b)) {
                    self.connections.push((index_a, index_b));
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn disconnect_nodes(&mut self, node_name_a: &str, node_name_b: &str) -> bool {
        if let Some(index_a) = self.nodes.iter().position(|n| n == node_name_a) {
            if let Some(index_b) = self.nodes.iter().position(|n| n == node_name_b) {
                if let Some(position) = self.connections.iter().position(|&(a, b)| a == index_a && b == index_b) {
                    self.connections.remove(position);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }
}
