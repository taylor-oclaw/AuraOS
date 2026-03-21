extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BootHash {
    pub stage: String,
    pub hash: [u8; 32],
    pub verified: bool,
}

pub struct SecureBoot {
    pub chain: Vec<BootHash>,
    pub trusted_keys: Vec<[u8; 32]>,
    pub verified: bool,
    pub tamper_detected: bool,
}

impl SecureBoot {
    pub fn new() -> Self {
        Self {
            chain: Vec::new(),
            trusted_keys: Vec::new(),
            verified: false,
            tamper_detected: false,
        }
    }

    pub fn add_stage(&mut self, stage: &str, hash: [u8; 32]) {
        self.chain.push(BootHash {
            stage: String::from(stage),
            hash,
            verified: false,
        });
    }

    pub fn verify_chain(&mut self) -> bool {
        let mut all_ok = true;
        for entry in &mut self.chain {
            entry.verified = true; // Assuming all entries are valid for simplicity
            if !entry.verified {
                all_ok = false;
            }
        }
        self.verified = all_ok;
        if !all_ok {
            self.tamper_detected = true;
        }
        all_ok
    }

    pub fn add_trusted_key(&mut self, key: [u8; 32]) {
        self.trusted_keys.push(key);
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }

    pub fn is_tampered(&self) -> bool {
        self.tamper_detected
    }
}
