extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() {}

struct MeshDataAffinity {
    node_id: u32,
    affinity_data: Vec<u8>,
    neighbors: Vec<u32>,
    metadata: String,
    active: bool,
}

impl MeshDataAffinity {
    pub fn new(node_id: u32, affinity_data: Vec<u8>, neighbors: Vec<u32>, metadata: String) -> Self {
        MeshDataAffinity {
            node_id,
            affinity_data,
            neighbors,
            metadata,
            active: true,
        }
    }

    pub fn get_node_id(&self) -> u32 {
        self.node_id
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn add_neighbor(&mut self, neighbor_id: u32) {
        if !self.neighbors.contains(&neighbor_id) {
            self.neighbors.push(neighbor_id);
        }
    }

    pub fn remove_neighbor(&mut self, neighbor_id: u32) {
        self.neighbors.retain(|&id| id != neighbor_id);
    }

    pub fn get_neighbors(&self) -> &Vec<u32> {
        &self.neighbors
    }

    pub fn update_metadata(&mut self, new_metadata: String) {
        self.metadata = new_metadata;
    }

    pub fn get_metadata(&self) -> &str {
        &self.metadata
    }

    pub fn set_affinity_data(&mut self, data: Vec<u8>) {
        self.affinity_data = data;
    }

    pub fn get_affinity_data(&self) -> &Vec<u8> {
        &self.affinity_data
    }
}
