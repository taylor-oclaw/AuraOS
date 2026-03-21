extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingFacetimeBridge {
    participants: Vec<String>,
    is_active: bool,
}

impl MeetingFacetimeBridge {
    pub fn new() -> Self {
        MeetingFacetimeBridge {
            participants: Vec::new(),
            is_active: false,
        }
    }

    pub fn start_meeting(&mut self) {
        if !self.is_active {
            self.is_active = true;
            // Logic to start the meeting
        }
    }

    pub fn end_meeting(&mut self) {
        if self.is_active {
            self.is_active = false;
            // Logic to end the meeting
        }
    }

    pub fn add_participant(&mut self, participant: String) {
        if !self.participants.contains(&participant) {
            self.participants.push(participant);
            // Logic to notify participants of new addition
        }
    }

    pub fn remove_participant(&mut self, participant: &str) {
        if let Some(index) = self.participants.iter().position(|p| p == participant) {
            self.participants.remove(index);
            // Logic to notify participants of removal
        }
    }

    pub fn list_participants(&self) -> Vec<String> {
        self.participants.clone()
    }
}
