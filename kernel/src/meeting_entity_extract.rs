extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingEntity {
    title: String,
    participants: Vec<String>,
    location: String,
    date: String,
    duration: u32, // in minutes
}

impl MeetingEntity {
    pub fn new(title: &str, participants: &[&str], location: &str, date: &str, duration: u32) -> Self {
        MeetingEntity {
            title: String::from(title),
            participants: participants.iter().map(|&p| String::from(p)).collect(),
            location: String::from(location),
            date: String::from(date),
            duration,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn add_participant(&mut self, participant: &str) {
        self.participants.push(String::from(participant));
    }

    pub fn remove_participant(&mut self, participant: &str) {
        self.participants.retain(|p| p != participant);
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = String::from(location);
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn set_date(&mut self, date: &str) {
        self.date = String::from(date);
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }
}
