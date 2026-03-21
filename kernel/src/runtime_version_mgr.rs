extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RuntimeVersionMgr {
    versions: Vec<String>,
}

impl RuntimeVersionMgr {
    pub fn new() -> Self {
        RuntimeVersionMgr {
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

    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }
}
