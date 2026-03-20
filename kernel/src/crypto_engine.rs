extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod crypto {
    use super::*;

    pub struct CryptoEngine {
        key: Vec<u8>,
    }

    impl CryptoEngine {
        pub fn new(key: &[u8]) -> Self {
            CryptoEngine {
                key: key.to_vec(),
            }
        }

        pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
            let mut ciphertext = plaintext.to_vec();
            for (i, byte) in ciphertext.iter_mut().enumerate() {
                *byte ^= self.key[i % self.key.len()];
            }
            ciphertext
        }

        pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
            self.encrypt(ciphertext)
        }

        pub fn generate_key(length: usize) -> Vec<u8> {
            (0..length).map(|_| rand::random::<u8>()).collect()
        }

        pub fn set_key(&mut self, key: &[u8]) {
            self.key = key.to_vec();
        }

        pub fn get_key_length(&self) -> usize {
            self.key.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::crypto::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = vec![0x1, 0x2, 0x3];
        let engine = CryptoEngine::new(&key);
        let plaintext = b"Hello, World!";
        let ciphertext = engine.encrypt(plaintext);
        let decrypted_text = engine.decrypt(&ciphertext);

        assert_eq!(decrypted_text, plaintext);
    }

    #[test]
    fn test_key_generation() {
        let key_length = 16;
        let generated_key = CryptoEngine::generate_key(key_length);
        assert_eq!(generated_key.len(), key_length);
    }
}
