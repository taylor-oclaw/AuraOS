extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_post_debrief_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_post_debrief_exit() {
    // Cleanup logic for the module
}

pub struct MeetingPostDebrief {
    participants: Vec<String>,
    topics_discussed: Vec<String>,
    action_items: Vec<String>,
    minutes_taken: String,
    next_meeting_date: Option<u64>, // Unix timestamp
}

impl MeetingPostDebrief {
    pub fn new() -> Self {
        MeetingPostDebrief {
            participants: Vec::new(),
            topics_discussed: Vec::new(),
            action_items: Vec::new(),
            minutes_taken: String::new(),
            next_meeting_date: None,
        }
    }

    pub fn add_participant(&mut self, participant: &str) {
        self.participants.push(String::from(participant));
    }

    pub fn add_topic_discussed(&mut self, topic: &str) {
        self.topics_discussed.push(String::from(topic));
    }

    pub fn add_action_item(&mut self, action_item: &str) {
        self.action_items.push(String::from(action_item));
    }

    pub fn set_minutes_taken(&mut self, minutes: &str) {
        self.minutes_taken = String::from(minutes);
    }

    pub fn set_next_meeting_date(&mut self, date: u64) {
        self.next_meeting_date = Some(date);
    }
}
