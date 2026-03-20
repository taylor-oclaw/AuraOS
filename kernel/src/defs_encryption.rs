extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EncryptionModule {
    key: u32,
}

impl EncryptionModule {
    pub fn new(key: u32) -> Self {
        EncryptionModule { key }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut encrypted = Vec::new();
        for &byte in data {
            encrypted.push(byte ^ self.key as u8);
        }
        encrypted
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut decrypted = Vec::new();
        for &byte in data {
            decrypted.push(byte ^ self.key as u8);
        }
        decrypted
    }

    pub fn encrypt_string(&self, data: &str) -> String {
        let encrypted_bytes = self.encrypt(data.as_bytes());
        unsafe { String::from_utf8_unchecked(encrypted_bytes) }
    }

    pub fn decrypt_string(&self, data: &str) -> String {
        let decrypted_bytes = self.decrypt(data.as_bytes());
        unsafe { String::from_utf8_unchecked(decrypted_bytes) }
    }

    pub fn change_key(&mut self, new_key: u32) {
        self.key = new_key;
    }
}
