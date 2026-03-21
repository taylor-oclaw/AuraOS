extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_mTLS_mesh_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_mTLS_mesh_exit() {
    // Cleanup logic for the module
}

pub struct MeshMTLSMesh {
    nodes: Vec<String>,
    connections: Vec<(String, String)>,
}

impl MeshMTLSMesh {
    pub fn new() -> Self {
        MeshMTLSMesh {
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

    pub fn connect_nodes(&mut self, from: &str, to: &str) -> bool {
        if self.nodes.contains(&from.to_string()) && self.nodes.contains(&to.to_string()) {
            self.connections.push((from.to_string(), to.to_string()));
            true
        } else {
            false
        }
    }

    pub fn disconnect_nodes(&mut self, from: &str, to: &str) -> bool {
        let pos = self.connections.iter().position(|&(ref a, ref b)| a == from && b == to);
        if let Some(p) = pos {
            self.connections.remove(p);
            true
        } else {
            false
        }
    }

    pub fn list_connections(&self) -> Vec<(String, String)> {
        self.connections.clone()
    }
}
