extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

pub struct DeadlockDetector {
    edges: Vec<(usize, usize)>,
    node_count: usize,
}

impl DeadlockDetector {
    pub fn new(node_count: usize) -> Self {
        DeadlockDetector { edges: Vec::new(), node_count }
    }

    pub fn add_dependency(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }

    pub fn remove_dependency(&mut self, from: usize, to: usize) {
        self.edges.retain(|&(f, t)| f != from || t != to);
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = vec![0u8; self.node_count]; // 0=unvisited, 1=in-progress, 2=done
        for node in 0..self.node_count {
            if visited[node] == 0 && self.dfs_cycle(node, &mut visited) {
                return true;
            }
        }
        false
    }

    fn dfs_cycle(&self, node: usize, visited: &mut Vec<u8>) -> bool {
        visited[node] = 1;
        for &(from, to) in &self.edges {
            if from == node {
                if visited[to] == 1 { return true; }
                if visited[to] == 0 && self.dfs_cycle(to, visited) { return true; }
            }
        }
        visited[node] = 2;
        false
    }

    pub fn dependency_count(&self) -> usize {
        self.edges.len()
    }

    pub fn clear(&mut self) {
        self.edges.clear();
    }
}
