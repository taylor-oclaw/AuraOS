extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HealthBreakReminder {
    reminders: Vec<String>,
    current_index: usize,
}

impl HealthBreakReminder {
    pub fn new() -> Self {
        HealthBreakReminder {
            reminders: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_reminder(&mut self, reminder: String) {
        self.reminders.push(reminder);
    }

    pub fn remove_reminder(&mut self, index: usize) -> Option<String> {
        if index < self.reminders.len() {
            Some(self.reminders.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_reminder(&self) -> Option<&String> {
        if self.reminders.is_empty() {
            None
        } else {
            Some(&self.reminders[self.current_index])
        }
    }

    pub fn next_reminder(&mut self) -> Option<&String> {
        if self.reminders.is_empty() {
            None
        } else {
            self.current_index = (self.current_index + 1) % self.reminders.len();
            Some(&self.reminders[self.current_index])
        }
    }

    pub fn previous_reminder(&mut self) -> Option<&String> {
        if self.reminders.is_empty() {
            None
        } else {
            if self.current_index == 0 {
                self.current_index = self.reminders.len() - 1;
            } else {
                self.current_index -= 1;
            }
            Some(&self.reminders[self.current_index])
        }
    }
}
