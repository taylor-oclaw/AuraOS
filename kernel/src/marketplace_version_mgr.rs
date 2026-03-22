extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceVersionMgr {
    versions: Vec<String>,
}

impl MarketplaceVersionMgr {
    pub fn new() -> Self {
        MarketplaceVersionMgr { versions: Vec::new() }
    }

    pub fn add_version(&mut self, version: String) {
        self.versions.push(version);
    }

    pub fn get_versions(&self) -> &Vec<String> {
        &self.versions
    }

    pub fn get_latest_version(&self) -> Option<&String> {
        if !self.versions.is_empty() {
            Some(self.versions.last().unwrap())
        } else {
            None
        }
    }

    pub fn update_versions(&mut self, new_versions: Vec<String>) {
        self.versions = new_versions;
    }

    pub fn remove_version(&mut self, version: &String) -> bool {
        if let Some(index) = self.versions.iter().position(|v| v == version) {
            self.versions.remove(index);
            true
        } else {
            false
        }
    }
}