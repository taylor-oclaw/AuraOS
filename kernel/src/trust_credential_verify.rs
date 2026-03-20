extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct TrustCredentialVerify {
    credentials: Vec<String>,
}

impl TrustCredentialVerify {
    pub fn new() -> Self {
        TrustCredentialVerify {
            credentials: Vec::new(),
        }
    }

    pub fn add_credential(&mut self, credential: String) {
        self.credentials.push(credential);
    }

    pub fn remove_credential(&mut self, index: usize) -> Option<String> {
        if index < self.credentials.len() {
            Some(self.credentials.remove(index))
        } else {
            None
        }
    }

    pub fn get_credentials(&self) -> &Vec<String> {
        &self.credentials
    }

    pub fn verify_credential(&self, credential: &str) -> bool {
        self.credentials.contains(&String::from(credential))
    }

    pub fn clear_credentials(&mut self) {
        self.credentials.clear();
    }
}
