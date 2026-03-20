extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct WebOfTrust {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>, // (from_node_index, to_node_index)
}

impl WebOfTrust {
    pub fn new() -> Self {
        WebOfTrust {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        let node = String::from(node_name);
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, from_node: &str, to_node: &str) -> bool {
        if let Some(from_index) = self.nodes.iter().position(|n| n == from_node) {
            if let Some(to_index) = self.nodes.iter().position(|n| n == to_node) {
                self.edges.push((from_index, to_index));
                return true;
            }
        }
        false
    }

    pub fn get_nodes(&self) -> &Vec<String> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }

    pub fn is_connected(&self, from_node: &str, to_node: &str) -> bool {
        if let Some(from_index) = self.nodes.iter().position(|n| n == from_node) {
            if let Some(to_index) = self.nodes.iter().position(|n| n == to_node) {
                return self.dfs(from_index, to_index);
            }
        }
        false
    }

    fn dfs(&self, current: usize, target: usize) -> bool {
        let mut visited = vec![false; self.nodes.len()];
        self.dfs_helper(current, target, &mut visited)
    }

    fn dfs_helper(&self, current: usize, target: usize, visited: &mut [bool]) -> bool {
        if visited[current] {
            return false;
        }
        visited[current] = true;

        if current == target {
            return true;
        }

        for &(from, to) in &self.edges {
            if from == current && self.dfs_helper(to, target, visited) {
                return true;
            }
        }

        false
    }
}
