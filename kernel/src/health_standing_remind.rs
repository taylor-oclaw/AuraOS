extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HealthStandingRemind {
    reminders: Vec<String>,
    interval_minutes: u32,
    last_reminded_time: u64,
}

impl HealthStandingRemind {
    pub fn new(interval_minutes: u32) -> Self {
        HealthStandingRemind {
            reminders: Vec::new(),
            interval_minutes,
            last_reminded_time: 0,
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

    pub fn get_reminders(&self) -> &Vec<String> {
        &self.reminders
    }

    pub fn set_interval_minutes(&mut self, interval_minutes: u32) {
        self.interval_minutes = interval_minutes;
    }

    pub fn check_and_remind(&mut self, current_time: u64) -> Option<&String> {
        if (current_time - self.last_reminded_time) >= (self.interval_minutes * 60) && !self.reminders.is_empty() {
            let reminder_index = (current_time % self.reminders.len() as u64) as usize;
            self.last_reminded_time = current_time;
            Some(&self.reminders[reminder_index])
        } else {
            None
        }
    }
}
