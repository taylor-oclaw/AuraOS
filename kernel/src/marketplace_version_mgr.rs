extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MarketplaceVersionMgr {
    versions: Vec<String>,
}

impl MarketplaceVersionMgr {
    pub fn new() -> Self {
        MarketplaceVersionMgr {
            versions: Vec::new(),
        }
    }

    pub fn add_version(&mut self, version: String) {
        if !self.versions.contains(&version) {
            self.versions.push(version);
        }
    }

    pub fn remove_version(&mut self, version: &str) -> bool {
        let pos = self.versions.iter().position(|v| v == version);
        if let Some(index) = pos {
            self.versions.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_versions(&self) -> &[String] {
        &self.versions
    }

    pub fn has_version(&self, version: &str) -> bool {
        self.versions.contains(&version.to_string())
    }

    pub fn count_versions(&self) -> usize {
        self.versions.len()
    }
}
