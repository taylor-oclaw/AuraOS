extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SessionManager {
    sessions: Vec<Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: Vec::new(),
        }
    }

    pub fn create_session(&mut self, user_id: u32) -> usize {
        let session = Session {
            user_id,
            active: true,
            data: String::from(""),
        };
        self.sessions.push(session);
        self.sessions.len() - 1
    }

    pub fn get_session(&self, index: usize) -> Option<&Session> {
        self.sessions.get(index)
    }

    pub fn update_session_data(&mut self, index: usize, data: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(index) {
            session.data.push_str(data);
            true
        } else {
            false
        }
    }

    pub fn end_session(&mut self, index: usize) -> bool {
        if let Some(session) = self.sessions.get_mut(index) {
            session.active = false;
            true
        } else {
            false
        }
    }

    pub fn list_active_sessions(&self) -> Vec<usize> {
        self.sessions
            .iter()
            .enumerate()
            .filter(|&(_, s)| s.active)
            .map(|(i, _)| i)
            .collect()
    }
}

struct Session {
    user_id: u32,
    active: bool,
    data: String,
}
