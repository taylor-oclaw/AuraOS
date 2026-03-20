extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod aes_gcm {
    use core::ptr;
    use crypto_aead::{AeadInPlace, NewAead};
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

    pub struct EncryptionAESGCM {
        key: Key,
    }

    impl EncryptionAESGCM {
        pub fn new(key_bytes: &[u8]) -> Self {
            let key = Key::from_slice(key_bytes);
            EncryptionAESGCM { key }
        }

        pub fn encrypt(&self, nonce_bytes: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, String> {
            if nonce_bytes.len() != 12 {
                return Err(String::from("Nonce must be 12 bytes long"));
            }

            let nonce = Nonce::from_slice(nonce_bytes);
            let cipher = ChaCha20Poly1305::new(self.key);
            let mut ciphertext = plaintext.to_vec();
            let tag_len = cipher.nonce_len();

            // Ensure there is enough space for the tag
            ciphertext.resize(ciphertext.len() + tag_len, 0);

            match cipher.encrypt_in_place_detached(nonce, b"", &mut ciphertext) {
                Ok(tag) => {
                    ciphertext.extend_from_slice(&tag);
                    Ok(ciphertext)
                }
                Err(_) => Err(String::from("Encryption failed")),
            }
        }

        pub fn decrypt(&self, nonce_bytes: &[u8], ciphertext_with_tag: &[u8]) -> Result<Vec<u8>, String> {
            if nonce_bytes.len() != 12 {
                return Err(String::from("Nonce must be 12 bytes long"));
            }

            let nonce = Nonce::from_slice(nonce_bytes);
            let cipher = ChaCha20Poly1305::new(self.key);
            let mut ciphertext = ciphertext_with_tag.to_vec();
            let tag_len = cipher.nonce_len();

            if ciphertext.len() < tag_len {
                return Err(String::from("Ciphertext too short"));
            }

            let (ciphertext, tag) = ciphertext.split_at_mut(ciphertext.len() - tag_len);

            match cipher.decrypt_in_place_detached(nonce, b"", ciphertext, tag) {
                Ok(_) => Ok(ciphertext.to_vec()),
                Err(_) => Err(String::from("Decryption failed")),
            }
        }

        pub fn generate_key(&self) -> Vec<u8> {
            self.key.as_ref().to_vec()
        }

        pub fn key_size(&self) -> usize {
            self.key.len()
        }

        pub fn nonce_size(&self) -> usize {
            12
        }
    }
}

#[cfg(test)]
mod tests {
    use super::aes_gcm::EncryptionAESGCM;

    #[test]
    fn test_encrypt_decrypt() {
        let key = b"an example very very secret key!";
        let nonce = b"a unique nonce";
        let plaintext = b"some plaintext message";

        let encryptor = EncryptionAESGCM::new(key);
        let ciphertext = encryptor.encrypt(nonce, plaintext).unwrap();
        let decrypted_text = encryptor.decrypt(nonce, &ciphertext).unwrap();

        assert_eq!(decrypted_text, plaintext);
    }
}
