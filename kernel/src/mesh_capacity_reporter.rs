extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshCapacityReporter {
    node_id: String,
    capacity: u32,
    used_capacity: u32,
    neighbors: Vec<String>,
}

impl MeshCapacityReporter {
    pub fn new(node_id: String, capacity: u32) -> Self {
        MeshCapacityReporter {
            node_id,
            capacity,
            used_capacity: 0,
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: String) {
        if !self.neighbors.contains(&neighbor_id) {
            self.neighbors.push(neighbor_id);
        }
    }

    pub fn remove_neighbor(&mut self, neighbor_id: &str) {
        self.neighbors.retain(|n| n != neighbor_id);
    }

    pub fn allocate_capacity(&mut self, amount: u32) -> bool {
        if self.used_capacity + amount <= self.capacity {
            self.used_capacity += amount;
            true
        } else {
            false
        }
    }

    pub fn deallocate_capacity(&mut self, amount: u32) {
        if amount > self.used_capacity {
            self.used_capacity = 0;
        } else {
            self.used_capacity -= amount;
        }
    }

    pub fn get_available_capacity(&self) -> u32 {
        self.capacity - self.used_capacity
    }
}
