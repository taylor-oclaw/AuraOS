extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_idle_detector_init() -> i32 {
    // Initialization logic for the module
    0
}

pub extern "C" fn mesh_idle_detector_exit() {
    // Cleanup logic for the module
}

pub struct MeshIdleDetector {
    idle_time: u64,
    active_nodes: Vec<String>,
    threshold: u64,
}

impl MeshIdleDetector {
    pub fn new(threshold: u64) -> Self {
        MeshIdleDetector {
            idle_time: 0,
            active_nodes: Vec::new(),
            threshold,
        }
    }

    pub fn add_active_node(&mut self, node_id: &str) {
        if !self.active_nodes.contains(&node_id.to_string()) {
            self.active_nodes.push(node_id.to_string());
        }
    }

    pub fn remove_active_node(&mut self, node_id: &str) {
        self.active_nodes.retain(|n| n != node_id);
    }

    pub fn update_idle_time(&mut self, time_passed: u64) {
        self.idle_time += time_passed;
    }

    pub fn reset_idle_time(&mut self) {
        self.idle_time = 0;
    }

    pub fn is_mesh_idle(&self) -> bool {
        self.active_nodes.is_empty() && self.idle_time >= self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_idle_detector() {
        let mut detector = MeshIdleDetector::new(100);
        assert!(!detector.is_mesh_idle());

        detector.add_active_node("node1");
        assert!(!detector.is_mesh_idle());

        detector.remove_active_node("node1");
        assert!(!detector.is_mesh_idle());

        detector.update_idle_time(50);
        assert!(!detector.is_mesh_idle());

        detector.update_idle_time(60);
        assert!(detector.is_mesh_idle());
    }
}
