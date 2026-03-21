extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[allow(non_camel_case_types)]
pub struct security_device_fingerprint {
    fingerprint_data: Vec<u8>,
    device_id: String,
    is_authenticated: bool,
}

impl security_device_fingerprint {
    pub fn new(device_id: &str) -> Self {
        security_device_fingerprint {
            fingerprint_data: Vec::new(),
            device_id: String::from(device_id),
            is_authenticated: false,
        }
    }

    pub fn add_fingerprint_data(&mut self, data: &[u8]) {
        self.fingerprint_data.extend_from_slice(data);
    }

    pub fn authenticate(&mut self) -> bool {
        // Placeholder logic for authentication
        if !self.fingerprint_data.is_empty() && self.device_id.len() > 0 {
            self.is_authenticated = true;
        }
        self.is_authenticated
    }

    pub fn is_device_authenticated(&self) -> bool {
        self.is_authenticated
    }

    pub fn clear_fingerprint_data(&mut self) {
        self.fingerprint_data.clear();
    }

    pub fn get_device_id(&self) -> &str {
        &self.device_id
    }
}
