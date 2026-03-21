extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshIntrusionDetect {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl MeshIntrusionDetect {
    pub fn new() -> Self {
        MeshIntrusionDetect {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_name: String) {
        self.nodes.push(node_name);
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.edges.push((from, to));
        }
    }

    pub fn get_nodes(&self) -> &Vec<String> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }

    pub fn detect_intrusion(&self, start: usize, end: usize) -> bool {
        if start >= self.nodes.len() || end >= self.nodes.len() {
            return false;
        }
        let mut visited = vec![false; self.nodes.len()];
        self.dfs(start, end, &mut visited)
    }

    fn dfs(&self, current: usize, target: usize, visited: &mut Vec<bool>) -> bool {
        if current == target {
            return true;
        }
        visited[current] = true;
        for &(from, to) in &self.edges {
            if from == current && !visited[to] {
                if self.dfs(to, target, visited) {
                    return true;
                }
            }
        }
        false
    }
}
