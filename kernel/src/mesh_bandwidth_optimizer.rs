extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct MeshBandwidthOptimizer {
    nodes: Vec<String>,
    bandwidths: Vec<u32>,
}

impl MeshBandwidthOptimizer {
    pub fn new() -> Self {
        MeshBandwidthOptimizer {
            nodes: Vec::new(),
            bandwidths: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: String) {
        if !self.nodes.contains(&node_name) {
            self.nodes.push(node_name);
            self.bandwidths.push(0);
        }
    }

    pub fn set_bandwidth(&mut self, node_name: &str, bandwidth: u32) -> bool {
        if let Some(index) = self.nodes.iter().position(|n| n == node_name) {
            self.bandwidths[index] = bandwidth;
            true
        } else {
            false
        }
    }

    pub fn get_bandwidth(&self, node_name: &str) -> Option<u32> {
        self.nodes.iter().position(|n| n == node_name).map(|index| self.bandwidths[index])
    }

    pub fn optimize_bandwidth(&mut self) {
        if let Some(max_index) = self.bandwidths.iter().enumerate().max_by_key(|&(_, &b)| b).map(|(i, _)| i) {
            for (i, node) in self.nodes.iter().enumerate() {
                if i != max_index {
                    self.set_bandwidth(node, self.bandwidths[max_index] / 2);
                }
            }
        }
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }
}
