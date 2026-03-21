extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MdmEncryptionPolicy {
    encryption_enabled: bool,
    allowed_algorithms: Vec<String>,
    key_size: usize,
    max_key_rotation_interval: u32,
    data_retention_policy: String,
}

impl MdmEncryptionPolicy {
    pub fn new(
        encryption_enabled: bool,
        allowed_algorithms: Vec<&str>,
        key_size: usize,
        max_key_rotation_interval: u32,
        data_retention_policy: &str,
     -> Self {
        MdmEncryptionPolicy {
            encryption_enabled,
            allowed_algorithms: allowed_algorithms.into_iter().map(String::from).collect(),
            key_size,
            max_key_rotation_interval,
            data_retention_policy: String::from(data_retention_policy),
        }
    }

    pub fn is_encryption_enabled(&self) -> bool {
        self.encryption_enabled
    }

    pub fn set_encryption_enabled(&mut self, enabled: bool) {
        self.encryption_enabled = enabled;
    }

    pub fn get_allowed_algorithms(&self) -> &Vec<String> {
        &self.allowed_algorithms
    }

    pub fn add_allowed_algorithm(&mut self, algorithm: &str) {
        if !self.allowed_algorithms.contains(&String::from(algorithm)) {
            self.allowed_algorithms.push(String::from(algorithm));
        }
    }

    pub fn remove_allowed_algorithm(&mut self, algorithm: &str) {
        self.allowed_algorithms.retain(|a| a != algorithm);
    }

    pub fn get_key_size(&self) -> usize {
        self.key_size
    }

    pub fn set_key_size(&mut self, size: usize) {
        self.key_size = size;
    }

    pub fn get_max_key_rotation_interval(&self) -> u32 {
        self.max_key_rotation_interval
    }

    pub fn set_max_key_rotation_interval(&mut self, interval: u32) {
        self.max_key_rotation_interval = interval;
    }

    pub fn get_data_retention_policy(&self) -> &str {
        &self.data_retention_policy
    }

    pub fn set_data_retention_policy(&mut self, policy: &str) {
        self.data_retention_policy = String::from(policy);
    }
}
