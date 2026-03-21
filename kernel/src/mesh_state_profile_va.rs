extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_state_profile_va_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_state_profile_va_exit() {
    // Cleanup logic for the module
}

pub struct MeshStateProfileVA {
    node_id: u32,
    neighbors: Vec<u32>,
    state: String,
    metrics: Vec<f32>,
    timestamp: u64,
}

impl MeshStateProfileVA {
    pub fn new(node_id: u32) -> Self {
        MeshStateProfileVA {
            node_id,
            neighbors: Vec::new(),
            state: String::from("idle"),
            metrics: Vec::new(),
            timestamp: 0,
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: u32) {
        if !self.neighbors.contains(&neighbor_id) {
            self.neighbors.push(neighbor_id);
        }
    }

    pub fn remove_neighbor(&mut self, neighbor_id: u32) {
        self.neighbors.retain(|&id| id != neighbor_id);
    }

    pub fn update_state(&mut self, new_state: &str) {
        self.state = String::from(new_state);
    }

    pub fn add_metric(&mut self, metric_value: f32) {
        self.metrics.push(metric_value);
    }

    pub fn get_neighbors_count(&self) -> usize {
        self.neighbors.len()
    }
}
