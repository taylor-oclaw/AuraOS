extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_attestation_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_attestation_exit() {
    // Cleanup logic for the module
}

pub struct MeshAttestation {
    nodes: Vec<String>,
    trust_scores: Vec<u8>,
}

impl MeshAttestation {
    pub fn new() -> Self {
        MeshAttestation {
            nodes: Vec::new(),
            trust_scores: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: &str) {
        if !self.nodes.contains(&node_id.to_string()) {
            self.nodes.push(node_id.to_string());
            self.trust_scores.push(0);
        }
    }

    pub fn remove_node(&mut self, node_id: &str) {
        if let Some(index) = self.nodes.iter().position(|n| n == node_id) {
            self.nodes.remove(index);
            self.trust_scores.remove(index);
        }
    }

    pub fn update_trust_score(&mut self, node_id: &str, score: u8) {
        if let Some(index) = self.nodes.iter().position(|n| n == node_id) {
            self.trust_scores[index] = score;
        }
    }

    pub fn get_trust_score(&self, node_id: &str) -> Option<u8> {
        self.nodes.iter().position(|n| n == node_id).map(|index| self.trust_scores[index])
    }

    pub fn list_nodes(&self) -> Vec<&String> {
        self.nodes.iter().collect()
    }
}
