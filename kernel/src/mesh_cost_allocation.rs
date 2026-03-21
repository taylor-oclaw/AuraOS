extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn mesh_cost_allocation_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_cost_allocation_exit() {
    // Cleanup logic for the module
}

pub struct MeshCostAllocator {
    nodes: Vec<String>,
    costs: Vec<Vec<u32>>,
}

impl MeshCostAllocator {
    pub fn new(nodes: Vec<String>) -> Self {
        let num_nodes = nodes.len();
        let mut costs = vec![vec![0; num_nodes]; num_nodes];
        for i in 0..num_nodes {
            costs[i][i] = 1; // Diagonal cost is set to 1
        }
        MeshCostAllocator { nodes, costs }
    }

    pub fn add_node(&mut self, node: String) {
        let new_index = self.nodes.len();
        self.nodes.push(node);
        for row in &mut self.costs {
            row.push(0);
        }
        self.costs.push(vec![0; new_index + 1]);
        self.costs[new_index][new_index] = 1;
    }

    pub fn set_cost(&mut self, from: usize, to: usize, cost: u32) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.costs[from][to] = cost;
        }
    }

    pub fn get_cost(&self, from: usize, to: usize) -> Option<u32> {
        if from < self.nodes.len() && to < self.nodes.len() {
            Some(self.costs[from][to])
        } else {
            None
        }
    }

    pub fn find_shortest_path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        let num_nodes = self.nodes.len();
        if from >= num_nodes || to >= num_nodes {
            return None;
        }

        let mut distances = vec![u32::MAX; num_nodes];
        let mut previous = vec![None; num_nodes];
        distances[from] = 0;

        let mut unvisited = (0..num_nodes).collect::<Vec<_>>();

        while !unvisited.is_empty() {
            let current_index = unvisited.iter()
                .min_by_key(|&&node| distances[node])
                .copied()
                .unwrap();
            unvisited.retain(|&x| x != current_index);

            for neighbor in 0..num_nodes {
                if self.costs[current_index][neighbor] > 0 && !unvisited.contains(&neighbor) {
                    let distance = distances[current_index] + self.costs[current_index][neighbor];
                    if distance < distances[neighbor] {
                        distances[neighbor] = distance;
                        previous[neighbor] = Some(current_index);
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut current = to;
        while let Some(prev) = previous[current] {
            path.push(current);
            current = prev;
        }
        if path.contains(&from) {
            path.push(from);
            path.reverse();
            Some(path)
        } else {
            None
        }
    }
}
