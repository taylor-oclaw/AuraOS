extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mam_version_pin_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mam_version_pin_exit() {
    // Cleanup logic for the module
}

pub struct VersionPin {
    version: String,
    dependencies: Vec<String>,
    is_pinned: bool,
}

impl VersionPin {
    pub fn new(version: &str) -> Self {
        VersionPin {
            version: String::from(version),
            dependencies: Vec::new(),
            is_pinned: false,
        }
    }

    pub fn pin(&mut self) {
        self.is_pinned = true;
    }

    pub fn unpin(&mut self) {
        self.is_pinned = false;
    }

    pub fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(String::from(dependency));
    }

    pub fn remove_dependency(&mut self, dependency: &str) -> bool {
        if let Some(pos) = self.dependencies.iter().position(|d| d == dependency) {
            self.dependencies.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn list_dependencies(&self) -> &[String] {
        &self.dependencies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_pin() {
        let mut vp = VersionPin::new("1.0.0");
        assert_eq!(vp.get_version(), "1.0.0");
        assert!(!vp.is_pinned());

        vp.pin();
        assert!(vp.is_pinned());

        vp.unpin();
        assert!(!vp.is_pinned());

        vp.add_dependency("dep1");
        vp.add_dependency("dep2");
        assert_eq!(vp.list_dependencies(), &[String::from("dep1"), String::from("dep2")]);

        assert!(vp.remove_dependency("dep1"));
        assert_eq!(vp.list_dependencies(), &[String::from("dep2")]);
        assert!(!vp.remove_dependency("dep3"));
    }
}
