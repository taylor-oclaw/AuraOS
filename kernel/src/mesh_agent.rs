extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_agent_init() {
    // Initialization logic for the mesh agent module
}

pub extern "C" fn mesh_agent_exit() {
    // Cleanup logic for the mesh agent module
}

pub struct MeshAgent {
    nodes: Vec<String>,
    connections: Vec<(String, String)>,
}

impl MeshAgent {
    pub fn new() -> Self {
        MeshAgent {
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
        self.nodes.retain(|n| n != node_id);
        self.connections.retain(|&(ref a, ref b)| a != node_id && b != node_id);
    }

    pub fn connect_nodes(&mut self, node_a: &str, node_b: &str) -> bool {
        if self.nodes.contains(&node_a.to_string()) && self.nodes.contains(&node_b.to_string()) {
            self.connections.push((node_a.to_string(), node_b.to_string()));
            true
        } else {
            false
        }
    }

    pub fn disconnect_nodes(&mut self, node_a: &str, node_b: &str) -> bool {
        let index = self.connections.iter().position(|&(ref a, ref b)| a == node_a && b == node_b);
        if let Some(idx) = index {
            self.connections.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn list_connections(&self) -> Vec<(String, String)> {
        self.connections.clone()
    }
}
