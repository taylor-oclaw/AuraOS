extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_bandwidth_share_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_bandwidth_share_exit() {
    // Cleanup logic for the module
}

pub struct MeshBandwidthShare {
    nodes: Vec<String>,
    bandwidths: Vec<u32>,
}

impl MeshBandwidthShare {
    pub fn new(nodes: Vec<String>, bandwidths: Vec<u32>) -> Self {
        assert_eq!(nodes.len(), bandwidths.len());
        MeshBandwidthShare { nodes, bandwidths }
    }

    pub fn add_node(&mut self, node: String, bandwidth: u32) {
        self.nodes.push(node);
        self.bandwidths.push(bandwidth);
    }

    pub fn remove_node(&mut self, index: usize) -> Option<(String, u32)> {
        if index < self.nodes.len() {
            Some((self.nodes.remove(index), self.bandwidths.remove(index)))
        } else {
            None
        }
    }

    pub fn get_bandwidth(&self, node_name: &str) -> Option<u32> {
        self.nodes.iter().position(|n| n == node_name).map(|i| self.bandwidths[i])
    }

    pub fn total_bandwidth(&self) -> u32 {
        self.bandwidths.iter().sum()
    }

    pub fn average_bandwidth(&self) -> Option<u32> {
        if self.nodes.is_empty() {
            None
        } else {
            Some(self.total_bandwidth() / self.nodes.len() as u32)
        }
    }
}
