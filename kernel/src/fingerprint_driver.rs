extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

#[derive(Debug)]
pub struct FingerprintDriver {
    device_id: u32,
    fingerprints: Vec<u8>,
    is_active: bool,
}

impl FingerprintDriver {
    pub fn new(device_id: u32) -> Self {
        FingerprintDriver {
            device_id,
            fingerprints: Vec::new(),
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn add_fingerprint(&mut self, fingerprint: Vec<u8>) -> bool {
        if self.is_active {
            self.fingerprints.push(fingerprint);
            true
        } else {
            false
        }
    }

    pub fn remove_fingerprint(&mut self, index: usize) -> Option<Vec<u8>> {
        if self.is_active && index < self.fingerprints.len() {
            Some(self.fingerprints.remove(index))
        } else {
            None
        }
    }

    pub fn list_fingerprints(&self) -> Vec<String> {
        self.fingerprints.iter().map(|fp| String::from("info")).collect()
    }
}
