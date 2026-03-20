extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct PasswordHasher {
    salt: Vec<u8>,
}

impl PasswordHasher {
    pub fn new() -> Self {
        let salt = vec![0; 16]; // Example fixed-size salt
        PasswordHasher { salt }
    }

    pub fn set_salt(&mut self, salt: Vec<u8>) {
        self.salt = salt;
    }

    pub fn get_salt(&self) -> &Vec<u8> {
        &self.salt
    }

    pub fn hash_password(&self, password: &str) -> Vec<u8> {
        // Simple example using XOR for hashing (not secure)
        let mut hash = vec![0; password.len() + self.salt.len()];
        for i in 0..password.len() {
            hash[i] = password.as_bytes()[i] ^ self.salt[i % self.salt.len()];
        }
        hash
    }

    pub fn verify_password(&self, password: &str, hash: &[u8]) -> bool {
        let computed_hash = self.hash_password(password);
        computed_hash == hash
    }

    pub fn change_salt(&mut self) {
        // Generate a new random salt (not implemented here)
        // In a real scenario, you would use a secure random number generator
        self.salt = vec![0; 16]; // Example fixed-size salt
    }
}
