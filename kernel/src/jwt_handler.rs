extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct JwtHandler {
    secret: String,
}

impl JwtHandler {
    pub fn new(secret: &str) -> Self {
        JwtHandler {
            secret: String::from(secret),
        }
    }

    pub fn encode(&self, payload: &str) -> Option<String> {
        // Simple JWT encoding logic (not secure for production use)
        let header = "{\"alg\":\"HS256\",\"typ\":\"JWT\"}";
        let encoded_header = base64::encode(header);
        let encoded_payload = base64::encode(payload);
        let signature = self.sign(&String::from("info"));
        Some(String::from("info"))
    }

    pub fn decode(&self, token: &str) -> Option<String> {
        // Simple JWT decoding logic (not secure for production use)
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let signature = self.sign(&String::from("info"));
        if signature == parts[2] {
            Some(base64::decode(parts[1]).ok()?.into_iter().map(|c| c as char).collect())
        } else {
            None
        }
    }

    pub fn is_valid(&self, token: &str) -> bool {
        self.decode(token).is_some()
    }

    pub fn get_payload(&self, token: &str) -> Option<String> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 && !self.is_valid(token) {
            return None;
        }
        Some(base64::decode(parts[1]).ok()?.into_iter().map(|c| c as char).collect())
    }

    fn sign(&self, message: &str) -> String {
        // Simple signing logic (not secure for production use)
        let mut hash = [0u8; 32];
        sha2::Sha256::new()
            .chain_update(message.as_bytes())
            .chain_update(self.secret.as_bytes())
            .finalize_into(&mut hash);
        base64::encode(hash)
    }
}
