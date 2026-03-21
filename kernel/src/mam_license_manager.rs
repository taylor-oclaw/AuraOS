extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mam_license_manager_init() {
    // Initialization logic for the license manager module
}

pub extern "C" fn mam_license_manager_exit() {
    // Cleanup logic for the license manager module
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

    pub fn add_license(&mut self, license: String) -> bool {
        if !self.licenses.contains(&license) {
            self.licenses.push(license);
            true
        } else {
            false
        }
    }

    pub fn remove_license(&mut self, license: &str) -> bool {
        let index = self.licenses.iter().position(|l| l == license);
        if let Some(i) = index {
            self.licenses.remove(i);
            true
        } else {
            false
        }
    }

    pub fn has_license(&self, license: &str) -> bool {
        self.licenses.contains(&String::from(license))
    }

    pub fn list_licenses(&self) -> Vec<String> {
        self.licenses.clone()
    }

    pub fn count_licenses(&self) -> usize {
        self.licenses.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_manager() {
        let mut manager = LicenseManager::new();
        assert_eq!(manager.count_licenses(), 0);

        assert!(manager.add_license(String::from("LICENSE123")));
        assert_eq!(manager.count_licenses(), 1);
        assert!(manager.has_license("LICENSE123"));

        assert!(!manager.add_license(String::from("LICENSE123"))); // Duplicate
        assert_eq!(manager.count_licenses(), 1);

        assert!(manager.remove_license("LICENSE123"));
        assert_eq!(manager.count_licenses(), 0);
        assert!(!manager.has_license("LICENSE123"));

        let licenses = manager.list_licenses();
        assert!(licenses.is_empty());
    }
}
