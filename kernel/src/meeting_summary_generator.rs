extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingSummaryGenerator {
    topics: Vec<String>,
    attendees: Vec<String>,
    summary: String,
}

impl MeetingSummaryGenerator {
    pub fn new() -> Self {
        MeetingSummaryGenerator {
            topics: Vec::new(),
            attendees: Vec::new(),
            summary: String::new(),
        }
    }

    pub fn add_topic(&mut self, topic: &str) {
        self.topics.push(topic.to_string());
    }

    pub fn add_attendee(&mut self, attendee: &str) {
        self.attendees.push(attendee.to_string());
    }

    pub fn generate_summary(&mut self) {
        let mut summary = String::from("Meeting Summary:\n");

        if !self.topics.is_empty() {
            summary.push_str("Topics Discussed:\n");
            for (index, topic) in self.topics.iter().enumerate() {
                summary.push_str(&format!("{}. {}\n", index + 1, topic));
            }
        } else {
            summary.push_str("No topics were discussed.\n");
        }

        if !self.attendees.is_empty() {
            summary.push_str("\nAttendees:\n");
            for (index, attendee) in self.attendees.iter().enumerate() {
                summary.push_str(&format!("{}. {}\n", index + 1, attendee));
            }
        } else {
            summary.push_str("No attendees were present.\n");
        }

        self.summary = summary;
    }

    pub fn get_summary(&self) -> &str {
        &self.summary
    }

    pub fn clear(&mut self) {
        self.topics.clear();
        self.attendees.clear();
        self.summary.clear();
    }
}
