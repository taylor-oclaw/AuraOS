extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshAnomalyDetect {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
    anomalies: Vec<usize>,
}

impl MeshAnomalyDetect {
    pub fn new() -> Self {
        MeshAnomalyDetect {
            nodes: Vec::new(),
            edges: Vec::new(),
            anomalies: Vec::new(),
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

    pub fn detect_anomalies(&mut self) {
        // Simple anomaly detection logic: nodes with no edges
        for i in 0..self.nodes.len() {
            if !self.edges.iter().any(|&(a, b)| a == i || b == i) {
                self.anomalies.push(i);
            }
        }
    }

    pub fn get_anomalies(&self) -> &[usize] {
        &self.anomalies
    }

    pub fn clear_anomalies(&mut self) {
        self.anomalies.clear();
    }
}
