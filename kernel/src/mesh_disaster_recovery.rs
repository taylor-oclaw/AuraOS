extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshDisasterRecovery {
    nodes: Vec<String>,
    recovery_plan: String,
}

impl MeshDisasterRecovery {
    pub fn new() -> Self {
        MeshDisasterRecovery {
            nodes: Vec::new(),
            recovery_plan: String::from("Initial Recovery Plan"),
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        self.nodes.push(String::from(node_name));
    }

    pub fn remove_node(&mut self, node_name: &str) -> bool {
        if let Some(index) = self.nodes.iter().position(|n| n == node_name) {
            self.nodes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    pub fn update_recovery_plan(&mut self, new_plan: &str) {
        self.recovery_plan = String::from(new_plan);
    }

    pub fn get_recovery_plan(&self) -> String {
        self.recovery_plan.clone()
    }
}
