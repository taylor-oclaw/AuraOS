extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct MutualConnectFinder {
    nodes: Vec<String>,
    connections: Vec<(usize, usize)>,
}

impl MutualConnectFinder {
    pub fn new() -> Self {
        MutualConnectFinder {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: String) {
        self.nodes.push(node_name);
    }

    pub fn connect_nodes(&mut self, node1_index: usize, node2_index: usize) -> Result<(), &'static str> {
        if node1_index >= self.nodes.len() || node2_index >= self.nodes.len() {
            Err("Node index out of bounds")
        } else {
            self.connections.push((node1_index, node2_index));
            Ok(())
        }
    }

    pub fn are_mutually_connected(&self, node1_index: usize, node2_index: usize) -> Result<bool, &'static str> {
        if node1_index >= self.nodes.len() || node2_index >= self.nodes.len() {
            Err("Node index out of bounds")
        } else {
            Ok(self.connections.iter().any(|&(a, b)| (a == node1_index && b == node2_index) || (a == node2_index && b == node1_index)))
        }
    }

    pub fn get_all_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn get_connections(&self) -> Vec<(usize, usize)> {
        self.connections.clone()
    }
}
