extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_state_profile_ca_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_state_profile_ca_exit() {
    // Cleanup logic for the module
}

pub struct MeshStateProfileCA {
    node_id: String,
    neighbors: Vec<String>,
    state: String,
    profile_data: Vec<u8>,
    capabilities: Vec<String>,
}

impl MeshStateProfileCA {
    pub fn new(node_id: &str) -> Self {
        MeshStateProfileCA {
            node_id: String::from(node_id),
            neighbors: Vec::new(),
            state: String::from("idle"),
            profile_data: Vec::new(),
            capabilities: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: &str) {
        if !self.neighbors.contains(&String::from(neighbor_id)) {
            self.neighbors.push(String::from(neighbor_id));
        }
    }

    pub fn remove_neighbor(&mut self, neighbor_id: &str) {
        self.neighbors.retain(|n| n != neighbor_id);
    }

    pub fn update_state(&mut self, new_state: &str) {
        self.state = String::from(new_state);
    }

    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&String::from(capability)) {
            self.capabilities.push(String::from(capability));
        }
    }

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }

    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }

    pub fn get_neighbors(&self) -> &[String] {
        &self.neighbors
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }

    pub fn get_profile_data(&self) -> &[u8] {
        &self.profile_data
    }

    pub fn get_capabilities(&self) -> &[String] {
        &self.capabilities
    }
}
