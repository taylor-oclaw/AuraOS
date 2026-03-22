extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraDiskEncrypt {
    key: Vec<u8>,
}

impl AuraDiskEncrypt {
    pub fn new(key: Vec<u8>) -> Self {
        AuraDiskEncrypt { key }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut encrypted_data = Vec::with_capacity(data.len());
        for (i, &byte) in data.iter().enumerate() {
            encrypted_data.push(byte ^ self.key[i % self.key.len()]);
        }
        encrypted_data
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8> {
        let mut decrypted_data = Vec::with_capacity(encrypted_data.len());
        for (i, &byte) in encrypted_data.iter().enumerate() {
            decrypted_data.push(byte ^ self.key[i % self.key.len()]);
        }
        decrypted_data
    }

    pub fn change_key(&mut self, new_key: Vec<u8>) {
        self.key = new_key;
    }

    pub fn key_size(&self) -> usize {
        self.key.len()
    }

    pub fn is_valid_key(&self, data: &[u8]) -> bool {
        if data.len() < self.key.len() {
            return false;
        }
        let encrypted_data = self.encrypt(data);
        let decrypted_data = self.decrypt(&encrypted_data);
        data == &decrypted_data
    }
}