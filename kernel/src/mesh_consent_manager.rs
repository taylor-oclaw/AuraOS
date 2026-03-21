extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_mesh_consent_manager() -> *mut MeshConsentManager {
    let manager = Box::new(MeshConsentManager::new());
    Box::leak(manager) as *mut MeshConsentManager
}

pub extern "C" fn destroy_mesh_consent_manager(ptr: *mut MeshConsentManager) {
    unsafe { drop(Box::from_raw(ptr)) }
}

pub struct MeshConsentManager {
    nodes: Vec<String>,
    permissions: Vec<(String, String)>, // (node_id, permission)
}

impl MeshConsentManager {
    pub fn new() -> Self {
        MeshConsentManager {
            nodes: Vec::new(),
            permissions: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: &str) {
        if !self.nodes.contains(&node_id.to_string()) {
            self.nodes.push(node_id.to_string());
        }
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.retain(|n| n != node_id);
        self.permissions.retain(|&(ref n, _)| n != node_id);
    }

    pub fn grant_permission(&mut self, node_id: &str, permission: &str) -> bool {
        if self.nodes.contains(&node_id.to_string()) {
            self.permissions.push((node_id.to_string(), permission.to_string()));
            true
        } else {
            false
        }
    }

    pub fn revoke_permission(&mut self, node_id: &str, permission: &str) -> bool {
        let index = self.permissions.iter().position(|&(ref n, ref p)| n == node_id && p == permission);
        if let Some(i) = index {
            self.permissions.remove(i);
            true
        } else {
            false
        }
    }

    pub fn has_permission(&self, node_id: &str, permission: &str) -> bool {
        self.permissions.iter().any(|&(ref n, ref p)| n == node_id && p == permission)
    }
}
