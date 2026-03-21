extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshLatencyOptimizer {
    nodes: Vec<String>,
    edges: Vec<(usize, usize, u32)>, // (node1_index, node2_index, latency)
}

impl MeshLatencyOptimizer {
    pub fn new() -> Self {
        MeshLatencyOptimizer {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, name: String) {
        self.nodes.push(name);
    }

    pub fn add_edge(&mut self, node1_name: &str, node2_name: &str, latency: u32) -> Result<(), &'static str> {
        let node1_index = self.nodes.iter().position(|n| n == node1_name).ok_or("Node 1 not found")?;
        let node2_index = self.nodes.iter().position(|n| n == node2_name).ok_or("Node 2 not found")?;
        self.edges.push((node1_index, node2_index, latency));
        Ok(())
    }

    pub fn get_latency(&self, node1_name: &str, node2_name: &str) -> Result<u32, &'static str> {
        let node1_index = self.nodes.iter().position(|n| n == node1_name).ok_or("Node 1 not found")?;
        let node2_index = self.nodes.iter().position(|n| n == node2_name).ok_or("Node 2 not found")?;
        for &(n1, n2, latency) in &self.edges {
            if (n1 == node1_index && n2 == node2_index) || (n1 == node2_index && n2 == node1_index) {
                return Ok(latency);
            }
        }
        Err("Edge not found")
    }

    pub fn find_shortest_path(&self, start_name: &str, end_name: &str) -> Result<Vec<String>, &'static str> {
        let start_index = self.nodes.iter().position(|n| n == start_name).ok_or("Start node not found")?;
        let end_index = self.nodes.iter().position(|n| n == end_name).ok_or("End node not found")?;

        let mut distances = vec![u32::MAX; self.nodes.len()];
        let mut previous_nodes = vec![None; self.nodes.len()];
        distances[start_index] = 0;

        for _ in 0..self.nodes.len() {
            let (current_distance, current_index) = distances.iter()
                .enumerate()
                .filter(|&(_, &dist)| dist != u32::MAX)
                .min_by_key(|&(_, &dist)| dist)
                .ok_or("No path found")?;

            if current_index == end_index {
                break;
            }

            for &(n1, n2, latency) in &self.edges {
                let neighbor = if n1 == current_index { n2 } else { n1 };
                let new_distance = current_distance + latency;
                if new_distance < distances[neighbor] {
                    distances[neighbor] = new_distance;
                    previous_nodes[neighbor] = Some(current_index);
                }
            }

            distances[current_index] = u32::MAX; // Mark as visited
        }

        let mut path = Vec::new();
        let mut current = end_index;
        while let Some(&prev) = previous_nodes[current] {
            path.push(self.nodes[prev].clone());
            if prev == start_index {
                break;
            }
            current = prev;
        }
        path.reverse();
        path.insert(0, self.nodes[end_index].clone());

        Ok(path)
    }

    pub fn remove_node(&mut self, name: &str) -> Result<(), &'static str> {
        let index = self.nodes.iter().position(|n| n == name).ok_or("Node not found")?;
        self.nodes.remove(index);
        self.edges.retain(|&(n1, n2, _)| n1 != index && n2 != index);
        Ok(())
    }
}
