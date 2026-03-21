extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshActivePassive {
    active_nodes: Vec<String>,
    passive_nodes: Vec<String>,
}

impl MeshActivePassive {
    pub fn new() -> Self {
        MeshActivePassive {
            active_nodes: Vec::new(),
            passive_nodes: Vec::new(),
        }
    }

    pub fn add_active_node(&mut self, node_id: String) {
        if !self.active_nodes.contains(&node_id) {
            self.active_nodes.push(node_id);
        }
    }

    pub fn add_passive_node(&mut self, node_id: String) {
        if !self.passive_nodes.contains(&node_id) {
            self.passive_nodes.push(node_id);
        }
    }

    pub fn remove_active_node(&mut self, node_id: &str) -> bool {
        let index = self.active_nodes.iter().position(|x| x == node_id);
        match index {
            Some(i) => {
                self.active_nodes.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn remove_passive_node(&mut self, node_id: &str) -> bool {
        let index = self.passive_nodes.iter().position(|x| x == node_id);
        match index {
            Some(i) => {
                self.passive_nodes.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn get_active_nodes(&self) -> &Vec<String> {
        &self.active_nodes
    }

    pub fn get_passive_nodes(&self) -> &Vec<String> {
        &self.passive_nodes
    }
}
