extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BizMeetingAttendeeBrief {
    name: String,
    email: String,
    role: String,
    is_important: bool,
    attended_meetings_count: u32,
}

impl BizMeetingAttendeeBrief {
    pub fn new(name: &str, email: &str, role: &str) -> Self {
        BizMeetingAttendeeBrief {
            name: String::from(name),
            email: String::from(email),
            role: String::from(role),
            is_important: false,
            attended_meetings_count: 0,
        }
    }

    pub fn mark_as_important(&mut self) {
        self.is_important = true;
    }

    pub fn unmark_as_important(&mut self) {
        self.is_important = false;
    }

    pub fn increment_attended_meetings(&mut self) {
        self.attended_meetings_count += 1;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn is_important(&self) -> bool {
        self.is_important
    }

    pub fn attended_meetings_count(&self) -> u32 {
        self.attended_meetings_count
    }
}
