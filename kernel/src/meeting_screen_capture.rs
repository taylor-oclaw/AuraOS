extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingScreenCapture {
    session_id: String,
    participants: Vec<String>,
    is_active: bool,
    capture_buffer: Vec<u8>,
}

impl MeetingScreenCapture {
    pub fn new(session_id: &str) -> Self {
        MeetingScreenCapture {
            session_id: String::from(session_id),
            participants: Vec::new(),
            is_active: false,
            capture_buffer: Vec::new(),
        }
    }

    pub fn start_capture(&mut self) {
        if !self.is_active {
            self.is_active = true;
            // Simulate starting screen capture
            self.capture_buffer.resize(1024 * 768, 0); // Example buffer size for a 1024x768 screen
        }
    }

    pub fn stop_capture(&mut self) {
        if self.is_active {
            self.is_active = false;
            // Simulate stopping screen capture
            self.capture_buffer.clear();
        }
    }

    pub fn add_participant(&mut self, participant: &str) {
        self.participants.push(String::from(participant));
    }

    pub fn remove_participant(&mut self, participant: &str) {
        if let Some(index) = self.participants.iter().position(|p| p == participant) {
            self.participants.remove(index);
        }
    }

    pub fn get_capture_buffer(&self) -> &[u8] {
        &self.capture_buffer
    }
}
