extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct CompatLinuxABI {
    version: String,
    features: Vec<String>,
}

impl CompatLinuxABI {
    pub fn new(version: &str) -> Self {
        CompatLinuxABI {
            version: String::from(version),
            features: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&String::from(feature))
    }

    pub fn list_features(&self) -> Vec<&String> {
        self.features.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let abi = CompatLinuxABI::new("1.0");
        assert_eq!(abi.version, "1.0");
        assert!(abi.features.is_empty());
    }

    #[test]
    fn test_add_feature() {
        let mut abi = CompatLinuxABI::new("1.0");
        abi.add_feature("feature1");
        assert_eq!(abi.features.len(), 1);
        assert_eq!(abi.features[0], "feature1");
    }

    #[test]
    fn test_get_version() {
        let abi = CompatLinuxABI::new("1.0");
        assert_eq!(abi.get_version(), &String::from("1.0"));
    }

    #[test]
    fn test_has_feature() {
        let mut abi = CompatLinuxABI::new("1.0");
        abi.add_feature("feature1");
        assert!(abi.has_feature("feature1"));
        assert!(!abi.has_feature("feature2"));
    }

    #[test]
    fn test_list_features() {
        let mut abi = CompatLinuxABI::new("1.0");
        abi.add_feature("feature1");
        abi.add_feature("feature2");
        let features = abi.list_features();
        assert_eq!(features.len(), 2);
        assert_eq!(features[0], &String::from("feature1"));
        assert_eq!(features[1], &String::from("feature2"));
    }
}
