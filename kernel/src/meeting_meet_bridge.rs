extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_meet_bridge_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_meet_bridge_exit() {
    // Cleanup logic for the module
}

pub struct MeetingMeetBridge {
    participants: Vec<String>,
    topic: String,
    duration: u32, // in minutes
    is_active: bool,
}

impl MeetingMeetBridge {
    pub fn new(topic: &str) -> Self {
        MeetingMeetBridge {
            participants: Vec::new(),
            topic: String::from(topic),
            duration: 0,
            is_active: false,
        }
    }

    pub fn start_meeting(&mut self, duration: u32) {
        if !self.is_active {
            self.duration = duration;
            self.is_active = true;
            // Additional logic to start the meeting
        }
    }

    pub fn end_meeting(&mut self) {
        if self.is_active {
            self.is_active = false;
            self.duration = 0;
            // Additional logic to end the meeting
        }
    }

    pub fn add_participant(&mut self, participant: &str) {
        if !self.participants.contains(&String::from(participant)) {
            self.participants.push(String::from(participant));
        }
    }

    pub fn remove_participant(&mut self, participant: &str) {
        self.participants.retain(|p| p != participant);
    }

    pub fn get_participants(&self) -> Vec<String> {
        self.participants.clone()
    }

    pub fn is_meeting_active(&self) -> bool {
        self.is_active
    }
}
