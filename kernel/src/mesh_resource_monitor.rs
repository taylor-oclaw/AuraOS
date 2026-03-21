extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshResourceMonitor {
    resources: Vec<String>,
}

impl MeshResourceMonitor {
    pub fn new() -> Self {
        MeshResourceMonitor {
            resources: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, resource_name: &str) {
        self.resources.push(String::from(resource_name));
    }

    pub fn remove_resource(&mut self, resource_name: &str) {
        if let Some(index) = self.resources.iter().position(|r| r == resource_name) {
            self.resources.remove(index);
        }
    }

    pub fn list_resources(&self) -> Vec<String> {
        self.resources.clone()
    }

    pub fn has_resource(&self, resource_name: &str) -> bool {
        self.resources.contains(&String::from(resource_name))
    }

    pub fn count_resources(&self) -> usize {
        self.resources.len()
    }
}
