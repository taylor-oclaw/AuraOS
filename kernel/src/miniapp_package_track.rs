extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct MiniAppPackageTrack {
    packages: Vec<String>,
}

impl MiniAppPackageTrack {
    pub fn new() -> Self {
        MiniAppPackageTrack {
            packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, package_name: &str) {
        self.packages.push(String::from(package_name));
    }

    pub fn remove_package(&mut self, package_name: &str) {
        if let Some(index) = self.packages.iter().position(|p| p == package_name) {
            self.packages.remove(index);
        }
    }

    pub fn list_packages(&self) -> Vec<String> {
        self.packages.clone()
    }

    pub fn contains_package(&self, package_name: &str) -> bool {
        self.packages.contains(&String::from(package_name))
    }

    pub fn count_packages(&self) -> usize {
        self.packages.len()
    }
}
