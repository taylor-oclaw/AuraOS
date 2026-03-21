extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct MeetingClientBrief {
    title: String,
    participants: Vec<String>,
    location: String,
    date: String,
    duration_hours: u32,
}

impl MeetingClientBrief {
    pub fn new(title: &str, participants: &[&str], location: &str, date: &str, duration_hours: u32) -> Self {
        MeetingClientBrief {
            title: String::from(title),
            participants: participants.iter().map(|&p| String::from(p)).collect(),
            location: String::from(location),
            date: String::from(date),
            duration_hours,
        }
    }

    pub fn add_participant(&mut self, participant: &str) {
        self.participants.push(String::from(participant));
    }

    pub fn remove_participant(&mut self, participant: &str) -> bool {
        if let Some(index) = self.participants.iter().position(|p| p == participant) {
            self.participants.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_participants(&self) -> &[String] {
        &self.participants
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn get_duration_hours(&self) -> u32 {
        self.duration_hours
    }
}
