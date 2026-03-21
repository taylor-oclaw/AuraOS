extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug)]
pub struct PeopleMeeting {
    participants: Vec<String>,
    location: String,
    date: String,
}

impl PeopleMeeting {
    pub fn new(location: &str, date: &str) -> Self {
        PeopleMeeting {
            participants: Vec::new(),
            location: String::from(location),
            date: String::from(date),
        }
    }

    pub fn add_participant(&mut self, name: &str) {
        self.participants.push(String::from(name));
    }

    pub fn remove_participant(&mut self, name: &str) -> bool {
        if let Some(index) = self.participants.iter().position(|x| x == name) {
            self.participants.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = String::from(location);
    }

    pub fn list_participants(&self) -> Vec<&str> {
        self.participants.iter().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_meeting() {
        let mut meeting = PeopleMeeting::new("Conference Room", "2023-10-01");
        assert_eq!(meeting.get_location(), "Conference Room");

        meeting.add_participant("Alice");
        meeting.add_participant("Bob");
        assert_eq!(meeting.list_participants(), vec!["Alice", "Bob"]);

        assert!(meeting.remove_participant("Alice"));
        assert_eq!(meeting.list_participants(), vec!["Bob"]);

        meeting.set_location("Meeting Hall");
        assert_eq!(meeting.get_location(), "Meeting Hall");

        assert!(!meeting.remove_participant("Charlie"));
    }
}
