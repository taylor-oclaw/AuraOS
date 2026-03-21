extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailScheduler {
    schedule: Vec<(u64, String)>, // (timestamp, email_content)
}

impl EmailScheduler {
    pub fn new() -> Self {
        EmailScheduler {
            schedule: Vec::new(),
        }
    }

    pub fn add_email(&mut self, timestamp: u64, content: String) {
        self.schedule.push((timestamp, content));
    }

    pub fn remove_email(&mut self, index: usize) -> Option<String> {
        if index < self.schedule.len() {
            Some(self.schedule.remove(index).1)
        } else {
            None
        }
    }

    pub fn get_emails_at_timestamp(&self, timestamp: u64) -> Vec<&String> {
        self.schedule
            .iter()
            .filter(|&&(t, _)| t == timestamp)
            .map(|&(_, ref content)| content)
            .collect()
    }

    pub fn list_all_emails(&self) -> Vec<&String> {
        self.schedule.iter().map(|&(_, ref content)| content).collect()
    }

    pub fn clear_schedule(&mut self) {
        self.schedule.clear();
    }
}
