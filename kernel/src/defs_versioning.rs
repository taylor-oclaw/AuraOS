extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

mod defs_versioning {
    use super::*;

    pub struct VersionManager {
        versions: Vec<String>,
    }

    impl VersionManager {
        pub fn new() -> Self {
            VersionManager {
                versions: Vec::new(),
            }
        }

        pub fn add_version(&mut self, version: &str) {
            self.versions.push(String::from(version));
        }

        pub fn get_latest_version(&self) -> Option<&String> {
            self.versions.last()
        }

        pub fn remove_version(&mut self, version: &str) -> bool {
            if let Some(index) = self.versions.iter().position(|v| v == version) {
                self.versions.remove(index);
                true
            } else {
                false
            }
        }

        pub fn list_versions(&self) -> Vec<&String> {
            self.versions.iter().collect()
        }

        pub fn has_version(&self, version: &str) -> bool {
            self.versions.contains(&String::from(version))
        }
    }
}
