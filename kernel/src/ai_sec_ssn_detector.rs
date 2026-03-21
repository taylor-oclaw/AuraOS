extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

#[derive(Debug)]
pub struct AISessionDetector {
    sessions: Vec<String>,
    max_sessions: usize,
}

impl AISessionDetector {
    pub fn new(max_sessions: usize) -> Self {
        AISessionDetector {
            sessions: Vec::new(),
            max_sessions,
        }
    }

    pub fn add_session(&mut self, session_id: &str) -> Result<(), &'static str> {
        if self.sessions.len() >= self.max_sessions {
            Err("Maximum number of sessions reached")
        } else {
            self.sessions.push(session_id.to_string());
            Ok(())
        }
    }

    pub fn remove_session(&mut self, session_id: &str) -> bool {
        let pos = self.sessions.iter().position(|s| s == session_id);
        if let Some(index) = pos {
            self.sessions.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_sessions(&self) -> Vec<String> {
        self.sessions.clone()
    }

    pub fn is_session_active(&self, session_id: &str) -> bool {
        self.sessions.contains(&session_id.to_string())
    }

    pub fn clear_sessions(&mut self) {
        self.sessions.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_session() {
        let mut detector = AISessionDetector::new(5);
        assert_eq!(detector.add_session("session1"), Ok(()));
        assert_eq!(detector.is_session_active("session1"), true);
        assert_eq!(detector.remove_session("session1"), true);
        assert_eq!(detector.is_session_active("session1"), false);
    }

    #[test]
    fn test_max_sessions() {
        let mut detector = AISessionDetector::new(2);
        assert_eq!(detector.add_session("session1"), Ok(()));
        assert_eq!(detector.add_session("session2"), Ok(()));
        assert_eq!(detector.add_session("session3"), Err("Maximum number of sessions reached"));
    }

    #[test]
    fn test_get_sessions() {
        let mut detector = AISessionDetector::new(5);
        detector.add_session("session1").unwrap();
        detector.add_session("session2").unwrap();
        let sessions = detector.get_sessions();
        assert_eq!(sessions.len(), 2);
        assert!(sessions.contains(&String::from("session1")));
        assert!(sessions.contains(&String::from("session2")));
    }

    #[test]
    fn test_clear_sessions() {
        let mut detector = AISessionDetector::new(5);
        detector.add_session("session1").unwrap();
        detector.clear_sessions();
        assert_eq!(detector.get_sessions().len(), 0);
    }
}
