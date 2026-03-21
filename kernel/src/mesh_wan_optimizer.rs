extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_wan_optimizer_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_wan_optimizer_exit() {
    // Cleanup logic for the module
}

pub struct MeshWanOptimizer {
    nodes: Vec<String>,
    routes: Vec<(String, String)>, // (source_node, destination_node)
    data_packets: u32,
    error_packets: u32,
    bandwidth_usage: f32,
}

impl MeshWanOptimizer {
    pub fn new() -> Self {
        MeshWanOptimizer {
            nodes: Vec::new(),
            routes: Vec::new(),
            data_packets: 0,
            error_packets: 0,
            bandwidth_usage: 0.0,
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        self.nodes.push(String::from(node_name));
    }

    pub fn remove_node(&mut self, node_name: &str) {
        self.nodes.retain(|n| n != node_name);
        self.routes.retain(|&(ref src, ref dst)| src != node_name && dst != node_name);
    }

    pub fn add_route(&mut self, source: &str, destination: &str) {
        if self.nodes.contains(&String::from(source)) && self.nodes.contains(&String::from(destination)) {
            self.routes.push((String::from(source), String::from(destination)));
        }
    }

    pub fn remove_route(&mut self, source: &str, destination: &str) {
        self.routes.retain(|&(ref src, ref dst)| src != source || dst != destination);
    }

    pub fn update_bandwidth_usage(&mut self, usage: f32) {
        self.bandwidth_usage = usage;
    }
}
