extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() {}

pub struct MeshStateProfileNy {
    node_id: String,
    neighbors: Vec<String>,
    data_packets_received: u32,
    data_packets_sent: u32,
    error_count: u32,
}

impl MeshStateProfileNy {
    pub fn new(node_id: &str) -> Self {
        MeshStateProfileNy {
            node_id: String::from(node_id),
            neighbors: Vec::new(),
            data_packets_received: 0,
            data_packets_sent: 0,
            error_count: 0,
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

    pub fn increment_received_packets(&mut self) {
        self.data_packets_received += 1;
    }

    pub fn increment_sent_packets(&mut self) {
        self.data_packets_sent += 1;
    }

    pub fn increment_error_count(&mut self) {
        self.error_count += 1;
    }

    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }

    pub fn get_neighbors(&self) -> &[String] {
        &self.neighbors
    }

    pub fn get_data_packets_received(&self) -> u32 {
        self.data_packets_received
    }

    pub fn get_data_packets_sent(&self) -> u32 {
        self.data_packets_sent
    }

    pub fn get_error_count(&self) -> u32 {
        self.error_count
    }
}
