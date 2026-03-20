extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EncryptionAes {
    key: Vec<u8>,
}

impl EncryptionAes {
    pub fn new(key: &[u8]) -> Self {
        EncryptionAes {
            key: key.to_vec(),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        if self.key.len() != 16 && self.key.len() != 24 && self.key.len() != 32 {
            return Err(String::from("Invalid key length"));
        }
        // Placeholder for encryption logic
        let mut ciphertext = plaintext.to_vec();
        // Simulate encryption by XORing with the key
        for (i, byte) in ciphertext.iter_mut().enumerate() {
            *byte ^= self.key[i % self.key.len()];
        }
        Ok(ciphertext)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        if self.key.len() != 16 && self.key.len() != 24 && self.key.len() != 32 {
            return Err(String::from("Invalid key length"));
        }
        // Placeholder for decryption logic
        let mut plaintext = ciphertext.to_vec();
        // Simulate decryption by XORing with the key
        for (i, byte) in plaintext.iter_mut().enumerate() {
            *byte ^= self.key[i % self.key.len()];
        }
        Ok(plaintext)
    }

    pub fn set_key(&mut self, new_key: &[u8]) -> Result<(), String> {
        if new_key.len() != 16 && new_key.len() != 24 && new_key.len() != 32 {
            return Err(String::from("Invalid key length"));
        }
        self.key = new_key.to_vec();
        Ok(())
    }

    pub fn get_key_length(&self) -> usize {
        self.key.len()
    }

    pub fn is_valid_key_length(&self, key_length: usize) -> bool {
        key_length == 16 || key_length == 24 || key_length == 32
    }
}
