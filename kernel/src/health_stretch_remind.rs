extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HealthStretchRemind {
    reminders: Vec<String>,
    current_index: usize,
}

impl HealthStretchRemind {
    pub fn new() -> Self {
        let mut reminders = Vec::new();
        reminders.push(String::from("Take a 5-minute break and stretch your arms."));
        reminders.push(String::from("Do some neck rotations for 30 seconds."));
        reminders.push(String::from("Stand up, walk around, and take deep breaths."));
        reminders.push(String::from("Stretch your legs by doing some leg swings."));
        reminders.push(String::from("Perform shoulder shrugs to relieve tension."));

        HealthStretchRemind {
            reminders,
            current_index: 0,
        }
    }

    pub fn get_next_reminder(&mut self) -> Option<&String> {
        if self.current_index >= self.reminders.len() {
            None
        } else {
            let reminder = &self.reminders[self.current_index];
            self.current_index += 1;
            Some(reminder)
        }
    }

    pub fn reset_reminders(&mut self) {
        self.current_index = 0;
    }

    pub fn add_reminder(&mut self, reminder: String) {
        self.reminders.push(reminder);
    }

    pub fn remove_last_reminder(&mut self) -> Option<String> {
        if self.reminders.is_empty() {
            None
        } else {
            Some(self.reminders.pop().unwrap())
        }
    }

    pub fn get_all_reminders(&self) -> &Vec<String> {
        &self.reminders
    }
}
