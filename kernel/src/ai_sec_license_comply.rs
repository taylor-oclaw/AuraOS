extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct LicenseManager {
    licenses: Vec<String>,
}

impl LicenseManager {
    pub fn new() -> Self {
        LicenseManager {
            licenses: Vec::new(),
        }
    }

    pub fn add_license(&mut self, license: String) {
        if !self.licenses.contains(&license) {
            self.licenses.push(license);
        }
    }

    pub fn remove_license(&mut self, license: &str) -> bool {
        let index = self.licenses.iter().position(|l| l == license);
        match index {
            Some(i) => {
                self.licenses.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn is_license_valid(&self, license: &str) -> bool {
        self.licenses.contains(&String::from(license))
    }

    pub fn list_licenses(&self) -> Vec<String> {
        self.licenses.clone()
    }

    pub fn count_licenses(&self) -> usize {
        self.licenses.len()
    }
}
