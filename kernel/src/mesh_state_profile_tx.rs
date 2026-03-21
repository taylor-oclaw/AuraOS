extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshStateProfileTx {
    node_id: String,
    neighbors: Vec<String>,
    tx_count: u32,
    last_tx_time: u64,
    error_rate: f32,
}

impl MeshStateProfileTx {
    pub fn new(node_id: &str) -> Self {
        MeshStateProfileTx {
            node_id: String::from(node_id),
            neighbors: Vec::new(),
            tx_count: 0,
            last_tx_time: 0,
            error_rate: 0.0,
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

    pub fn increment_tx_count(&mut self) {
        self.tx_count += 1;
    }

    pub fn update_last_tx_time(&mut self, current_time: u64) {
        self.last_tx_time = current_time;
    }

    pub fn calculate_error_rate(&mut self, successful_txs: u32) -> f32 {
        if self.tx_count == 0 {
            self.error_rate = 0.0;
        } else {
            self.error_rate = (self.tx_count - successful_txs) as f32 / self.tx_count as f32;
        }
        self.error_rate
    }

    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }

    pub fn get_neighbors(&self) -> &[String] {
        &self.neighbors
    }

    pub fn get_tx_count(&self) -> u32 {
        self.tx_count
    }

    pub fn get_last_tx_time(&self) -> u64 {
        self.last_tx_time
    }

    pub fn get_error_rate(&self) -> f32 {
        self.error_rate
    }
}
