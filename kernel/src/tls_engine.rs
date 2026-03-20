extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TlsEngine {
    certificates: Vec<String>,
    private_keys: Vec<String>,
    session_cache: Vec<(String, String)>, // (session_id, session_data)
}

impl TlsEngine {
    pub fn new() -> Self {
        TlsEngine {
            certificates: Vec::new(),
            private_keys: Vec::new(),
            session_cache: Vec::new(),
        }
    }

    pub fn add_certificate(&mut self, certificate: String) {
        self.certificates.push(certificate);
    }

    pub fn add_private_key(&mut self, private_key: String) {
        self.private_keys.push(private_key);
    }

    pub fn get_certificates(&self) -> &Vec<String> {
        &self.certificates
    }

    pub fn get_private_keys(&self) -> &Vec<String> {
        &self.private_keys
    }

    pub fn store_session(&mut self, session_id: String, session_data: String) {
        self.session_cache.push((session_id, session_data));
    }

    pub fn retrieve_session(&self, session_id: &str) -> Option<&String> {
        for (id, data) in &self.session_cache {
            if id == session_id {
                return Some(data);
            }
        }
        None
    }
}
