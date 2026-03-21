extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_itar_boundary_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_itar_boundary_exit() {
    // Cleanup logic for the module
}

pub struct MeshItarBoundary {
    nodes: Vec<(i32, i32)>,
    edges: Vec<(usize, usize)>,
}

impl MeshItarBoundary {
    pub fn new() -> Self {
        MeshItarBoundary {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, x: i32, y: i32) -> usize {
        let index = self.nodes.len();
        self.nodes.push((x, y));
        index
    }

    pub fn add_edge(&mut self, node1_index: usize, node2_index: usize) {
        if node1_index < self.nodes.len() && node2_index < self.nodes.len() {
            self.edges.push((node1_index, node2_index));
        }
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn get_boundary_nodes(&self) -> Vec<(i32, i32)> {
        let mut boundary_nodes = Vec::new();
        for (index, &(x, y)) in self.nodes.iter().enumerate() {
            if self.is_boundary_node(index) {
                boundary_nodes.push((x, y));
            }
        }
        boundary_nodes
    }

    fn is_boundary_node(&self, node_index: usize) -> bool {
        if let Some(&(node1, node2)) = self.edges.iter().find(|&&(n1, n2)| n1 == node_index || n2 == node_index) {
            return true;
        }
        false
    }
}
