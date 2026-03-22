extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyHubLegacyVault {
    key: String,
}

impl FamilyHubLegacyVault {
    pub fn new(key: String) -> Self {
        FamilyHubLegacyVault { key }
    }

    pub fn get_key(&self) -> &str {
        self.key.as_str()
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn encrypt_data(&self, data: Vec<u8>) -> Vec<u8> {
        // Simple XOR encryption for demonstration purposes
        let mut encrypted_data = Vec::new();
        for byte in data.iter() {
            encrypted_data.push(byte ^ self.key.as_bytes()[0]);
        }
        encrypted_data
    }

    pub fn decrypt_data(&self, data: Vec<u8>) -> Vec<u8> {
        // Simple XOR decryption for demonstration purposes
        let mut decrypted_data = Vec::new();
        for byte in data.iter() {
            decrypted_data.push(byte ^ self.key.as_bytes()[0]);
        }
        decrypted_data
    }

    pub fn generate_key(&self) -> String {
        // Generate a new key based on the current system time
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        format!("{:x}", now.as_millis())
    }
}