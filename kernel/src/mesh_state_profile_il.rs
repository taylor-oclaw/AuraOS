extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_state_profile_il_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_state_profile_il_exit() {
    // Cleanup logic for the module
}

pub struct MeshStateProfileIL {
    node_id: String,
    neighbors: Vec<String>,
    state: String,
    metrics: Vec<(String, u32)>,
    history: Vec<String>,
}

impl MeshStateProfileIL {
    pub fn new(node_id: &str) -> Self {
        MeshStateProfileIL {
            node_id: String::from(node_id),
            neighbors: Vec::new(),
            state: String::from("idle"),
            metrics: Vec::new(),
            history: Vec::new(),
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
        self.history.push(self.state.clone());
        self.state = String::from(new_state);
    }

    pub fn add_metric(&mut self, metric_name: &str, value: u32) {
        let metric = (String::from(metric_name), value);
        if !self.metrics.contains(&metric) {
            self.metrics.push(metric);
        }
    }

    pub fn get_neighbors_count(&self) -> usize {
        self.neighbors.len()
    }

    pub fn get_state_history(&self) -> Vec<String> {
        self.history.clone()
    }
}
