extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_multi_site_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_multi_site_exit() {
    // Cleanup logic for the module
}

pub struct MeshMultiSite {
    sites: Vec<String>,
    connections: Vec<(String, String)>,
}

impl MeshMultiSite {
    pub fn new() -> Self {
        MeshMultiSite {
            sites: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_site(&mut self, site_name: &str) {
        if !self.sites.contains(&site_name.to_string()) {
            self.sites.push(site_name.to_string());
        }
    }

    pub fn remove_site(&mut self, site_name: &str) {
        self.sites.retain(|s| s != site_name);
        self.connections.retain(|&(ref a, ref b)| a != site_name && b != site_name);
    }

    pub fn connect_sites(&mut self, from: &str, to: &str) -> bool {
        if self.sites.contains(&from.to_string()) && self.sites.contains(&to.to_string()) {
            self.connections.push((from.to_string(), to.to_string()));
            true
        } else {
            false
        }
    }

    pub fn disconnect_sites(&mut self, from: &str, to: &str) -> bool {
        let pos = self.connections.iter().position(|&(ref a, ref b)| a == from && b == to);
        if let Some(p) = pos {
            self.connections.remove(p);
            true
        } else {
            false
        }
    }

    pub fn list_connections(&self) -> Vec<(String, String)> {
        self.connections.clone()
    }
}
