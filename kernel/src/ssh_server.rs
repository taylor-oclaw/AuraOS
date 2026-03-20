extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SshServer {
    users: Vec<String>,
    sessions: Vec<Session>,
}

impl SshServer {
    pub fn new() -> Self {
        SshServer {
            users: Vec::new(),
            sessions: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: &str) {
        if !self.users.contains(&username.to_string()) {
            self.users.push(username.to_string());
        }
    }

    pub fn remove_user(&mut self, username: &str) {
        self.users.retain(|user| user != username);
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        // For simplicity, assume all users have the same password
        self.users.contains(&username.to_string())
    }

    pub fn start_session(&mut self, username: &str) -> Option<usize> {
        if self.authenticate(username, "") {
            let session_id = self.sessions.len();
            self.sessions.push(Session::new(username));
            Some(session_id)
        } else {
            None
        }
    }

    pub fn end_session(&mut self, session_id: usize) {
        if session_id < self.sessions.len() {
            self.sessions.remove(session_id);
        }
    }
}

struct Session {
    username: String,
    // Add more session-related fields as needed
}

impl Session {
    fn new(username: &str) -> Self {
        Session {
            username: username.to_string(),
        }
    }
}
