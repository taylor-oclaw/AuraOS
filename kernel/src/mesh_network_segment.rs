extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshNetworkSegment {
    nodes: Vec<String>,
    connections: Vec<(usize, usize)>,
}

impl MeshNetworkSegment {
    pub fn new() -> Self {
        MeshNetworkSegment {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: &str) {
        if !self.nodes.contains(&node_id.to_string()) {
            self.nodes.push(node_id.to_string());
        }
    }

    pub fn remove_node(&mut self, node_id: &str) {
        if let Some(index) = self.nodes.iter().position(|n| n == node_id) {
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
        }
    }

    pub fn connect_nodes(&mut self, node_id_a: &str, node_id_b: &str) {
        if let (Some(index_a), Some(index_b)) = (
            self.nodes.iter().position(|n| n == node_id_a),
            self.nodes.iter().position(|n| n == node_id_b),
        ) {
            if index_a != index_b && !self.connections.contains(&(index_a, index_b)) {
                self.connections.push((index_a, index_b));
            }
        }
    }

    pub fn disconnect_nodes(&mut self, node_id_a: &str, node_id_b: &str) {
        if let (Some(index_a), Some(index_b)) = (
            self.nodes.iter().position(|n| n == node_id_a),
            self.nodes.iter().position(|n| n == node_id_b),
        ) {
            self.connections.retain(|&(a, b)| a != index_a || b != index_b);
        }
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn list_connections(&self) -> Vec<(String, String)> {
        self.connections
            .iter()
            .map(|&(a, b)| (self.nodes[a].clone(), self.nodes[b].clone()))
            .collect()
    }
}
