extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshRetentionPolicy {
    max_nodes: usize,
    node_list: Vec<String>,
}

impl MeshRetentionPolicy {
    pub fn new(max_nodes: usize) -> Self {
        MeshRetentionPolicy {
            max_nodes,
            node_list: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: &str) -> bool {
        if self.node_list.len() < self.max_nodes {
            self.node_list.push(node_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn remove_node(&mut self, node_id: &str) -> bool {
        if let Some(index) = self.node_list.iter().position(|n| n == node_id) {
            self.node_list.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_nodes(&self) -> &[String] {
        &self.node_list
    }

    pub fn is_full(&self) -> bool {
        self.node_list.len() >= self.max_nodes
    }

    pub fn clear(&mut self) {
        self.node_list.clear();
    }
}
