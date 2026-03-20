extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct MCPAuthHandler {
    users: Vec<String>,
    sessions: Vec<(String, String)>, // (username, session_token)
}

impl MCPAuthHandler {
    pub fn new() -> Self {
        MCPAuthHandler {
            users: Vec::new(),
            sessions: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: &str) {
        if !self.users.contains(&String::from(username)) {
            self.users.push(String::from(username));
        }
    }

    pub fn remove_user(&mut self, username: &str) {
        self.users.retain(|u| u != username);
        self.sessions.retain(|(u, _)| u != username);
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> bool {
        if self.users.contains(&String::from(username)) {
            // Simulate password check
            let session_token = format!("token_{}", username); // Simplified token generation
            self.sessions.push((String::from(username), session_token));
            true
        } else {
            false
        }
    }

    pub fn validate_session(&self, session_token: &str) -> Option<&str> {
        for (username, token) in &self.sessions {
            if token == session_token {
                return Some(username);
            }
        }
        None
    }

    pub fn end_session(&mut self, session_token: &str) {
        self.sessions.retain(|(_, token)| token != session_token);
    }
}
