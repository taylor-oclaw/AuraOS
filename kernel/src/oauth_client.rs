extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OAuthClient {
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_expiry: u64, // Unix timestamp in seconds
}

impl OAuthClient {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        OAuthClient {
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
            access_token: None,
            refresh_token: None,
            token_expiry: 0,
        }
    }

    pub fn set_access_token(&mut self, token: &str, expiry: u64) {
        self.access_token = Some(String::from(token));
        self.token_expiry = expiry;
    }

    pub fn get_access_token(&self) -> Option<&String> {
        if let Some(ref token) = self.access_token {
            if self.token_expiry > 0 && self.token_expiry > current_time() {
                return Some(token);
            }
        }
        None
    }

    pub fn set_refresh_token(&mut self, token: &str) {
        self.refresh_token = Some(String::from(token));
    }

    pub fn get_refresh_token(&self) -> Option<&String> {
        self.refresh_token.as_ref()
    }

    pub fn is_token_expired(&self) -> bool {
        if let Some(_) = self.access_token {
            return self.token_expiry <= current_time();
        }
        true
    }
}

fn current_time() -> u64 {
    // Placeholder for getting the current time in seconds since epoch
    0
}
