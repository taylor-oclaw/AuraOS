extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingSlackHuddle {
    participants: Vec<String>,
    topic: String,
    duration_minutes: u32,
    scheduled_time: u64, // Unix timestamp in seconds
    is_active: bool,
}

impl MeetingSlackHuddle {
    pub fn new(topic: &str, scheduled_time: u64) -> Self {
        MeetingSlackHuddle {
            participants: Vec::new(),
            topic: String::from(topic),
            duration_minutes: 30, // Default duration
            scheduled_time,
            is_active: false,
        }
    }

    pub fn add_participant(&mut self, participant: &str) {
        self.participants.push(String::from(participant));
    }

    pub fn remove_participant(&mut self, participant: &str) {
        self.participants.retain(|p| p != participant);
    }

    pub fn set_duration(&mut self, duration_minutes: u32) {
        self.duration_minutes = duration_minutes;
    }

    pub fn start_meeting(&mut self) {
        if !self.is_active && self.scheduled_time <= current_time() {
            self.is_active = true;
        }
    }

    pub fn end_meeting(&mut self) {
        self.is_active = false;
    }

    pub fn get_participants(&self) -> &Vec<String> {
        &self.participants
    }

    pub fn get_topic(&self) -> &str {
        &self.topic
    }

    pub fn get_duration(&self) -> u32 {
        self.duration_minutes
    }

    pub fn is_meeting_active(&self) -> bool {
        self.is_active
    }
}

fn current_time() -> u64 {
    // Placeholder for actual time retrieval logic
    1672531200 // Example Unix timestamp
}
