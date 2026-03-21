extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut registry = MeshSiteRegistry::new();
    registry.register_site("site1".into(), "192.168.1.1".into());
    registry.register_site("site2".into(), "192.168.1.2".into());

    if let Some(ip) = registry.get_ip_by_name("site1") {
    }

    loop {}
}

pub struct MeshSiteRegistry {
    sites: Vec<(String, String)>,
}

impl MeshSiteRegistry {
    pub fn new() -> Self {
        MeshSiteRegistry { sites: Vec::new() }
    }

    pub fn register_site(&mut self, name: String, ip: String) {
        self.sites.push((name, ip));
    }

    pub fn get_ip_by_name(&self, name: &str) -> Option<&String> {
        for (site_name, ip) in &self.sites {
            if site_name == name {
                return Some(ip);
            }
        }
        None
    }

    pub fn remove_site_by_name(&mut self, name: &str) -> bool {
        let pos = self.sites.iter().position(|(site_name, _)| site_name == name);
        if let Some(index) = pos {
            self.sites.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_sites(&self) -> Vec<&String> {
        self.sites.iter().map(|(name, _)| name).collect()
    }

    pub fn count_sites(&self) -> usize {
        self.sites.len()
    }
}
