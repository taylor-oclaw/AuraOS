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
        let mut encrypted_data = Vec::new();
        for (i, &byte) in data.iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            encrypted_data.push(byte ^ key_byte);
        }
        encrypted_data
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8> {
        self.encrypt(encrypted_data)
    }

    pub fn set_key(&mut self, new_key: Vec<u8>) {
        self.key = new_key;
    }

    pub fn get_key_size(&self) -> usize {
        self.key.len()
    }

    pub fn is_key_empty(&self) -> bool {
        self.key.is_empty()
    }
}
