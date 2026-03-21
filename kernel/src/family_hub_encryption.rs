extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct FamilyHubEncryption {
    key: Vec<u8>,
}

impl FamilyHubEncryption {
    pub fn new(key: Vec<u8>) -> Self {
        FamilyHubEncryption { key }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut encrypted = Vec::new();
        for (i, &byte) in data.iter().enumerate() {
            encrypted.push(byte ^ self.key[i % self.key.len()]);
        }
        encrypted
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Vec<u8> {
        self.encrypt(encrypted_data)
    }

    pub fn set_key(&mut self, new_key: Vec<u8>) {
        self.key = new_key;
    }

    pub fn get_key(&self) -> &Vec<u8> {
        &self.key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = vec![0x1, 0x2, 0x3];
        let data = b"Hello, World!";
        let mut encryption = FamilyHubEncryption::new(key);

        let encrypted_data = encryption.encrypt(data);
        let decrypted_data = encryption.decrypt(&encrypted_data);

        assert_eq!(data.to_vec(), decrypted_data);
    }

    #[test]
    fn test_key_change() {
        let key1 = vec![0x1, 0x2, 0x3];
        let key2 = vec![0x4, 0x5, 0x6];
        let data = b"Hello, World!";
        let mut encryption = FamilyHubEncryption::new(key1);

        let encrypted_data_with_key1 = encryption.encrypt(data);
        encryption.set_key(key2);
        let encrypted_data_with_key2 = encryption.encrypt(data);

        assert_ne!(encrypted_data_with_key1, encrypted_data_with_key2);
    }
}
