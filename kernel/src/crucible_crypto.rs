extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn crucible_crypto_init() {
    // Initialization code for the module
}

pub extern "C" fn crucible_crypto_exit() {
    // Cleanup code for the module
}

pub struct CryptoModule {
    key: Vec<u8>,
}

impl CryptoModule {
    pub fn new(key: &[u8]) -> Self {
        CryptoModule {
            key: key.to_vec(),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        // Simple XOR encryption for demonstration purposes
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        for (i, &byte) in plaintext.iter().enumerate() {
            ciphertext.push(byte ^ self.key[i % self.key.len()]);
        }
        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        // Simple XOR decryption for demonstration purposes
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        for (i, &byte) in ciphertext.iter().enumerate() {
            plaintext.push(byte ^ self.key[i % self.key.len()]);
        }
        plaintext
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.key = key.to_vec();
    }

    pub fn get_key(&self) -> Vec<u8> {
        self.key.clone()
    }

    pub fn is_key_set(&self) -> bool {
        !self.key.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = b"secretkey";
        let crypto = CryptoModule::new(key);
        let plaintext = b"Hello, World!";
        let ciphertext = crypto.encrypt(plaintext);
        let decrypted_text = crypto.decrypt(&ciphertext);

        assert_eq!(decrypted_text, plaintext.to_vec());
    }

    #[test]
    fn test_key_management() {
        let key = b"initialkey";
        let mut crypto = CryptoModule::new(key);
        assert!(crypto.is_key_set());

        let new_key = b"newkey";
        crypto.set_key(new_key);
        assert_eq!(crypto.get_key(), new_key.to_vec());
    }
}
