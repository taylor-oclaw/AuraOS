extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn license_init() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn license_exit() -> i32 {
    0
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
        let pos = self.licenses.iter().position(|x| x == license);
        if let Some(index) = pos {
            self.licenses.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_valid_license(&self, license: &str) -> bool {
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

        let license1 = String::from("LICENSE123");
        let license2 = String::from("LICENSE456");

        assert!(manager.add_license(license1.clone()));
        assert!(!manager.add_license(license1.clone())); // Duplicate should not be added
        assert_eq!(manager.count_licenses(), 1);

        assert!(manager.add_license(license2.clone()));
        assert_eq!(manager.count_licenses(), 2);

        assert!(manager.is_valid_license("LICENSE123"));
        assert!(!manager.is_valid_license("INVALID_LICENSE"));

        let licenses = manager.list_licenses();
        assert_eq!(licenses.len(), 2);
        assert!(licenses.contains(&license1));
        assert!(licenses.contains(&license2));

        assert!(manager.remove_license("LICENSE123"));
        assert!(!manager.remove_license("LICENSE123")); // Already removed
        assert_eq!(manager.count_licenses(), 1);

        assert!(!manager.is_valid_license("LICENSE123"));
    }
}
