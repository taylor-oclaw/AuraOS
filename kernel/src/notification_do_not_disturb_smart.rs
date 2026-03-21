extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct NotificationDoNotDisturbSmart {
    enabled: bool,
    allowed_apps: Vec<String>,
    blocked_contacts: Vec<String>,
    schedule_start: u32, // in minutes since midnight
    schedule_end: u32,   // in minutes since midnight
}

impl NotificationDoNotDisturbSmart {
    pub fn new() -> Self {
        NotificationDoNotDisturbSmart {
            enabled: false,
            allowed_apps: Vec::new(),
            blocked_contacts: Vec::new(),
            schedule_start: 0,
            schedule_end: 1440, // 24 hours
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn add_allowed_app(&mut self, app_name: String) {
        if !self.allowed_apps.contains(&app_name) {
            self.allowed_apps.push(app_name);
        }
    }

    pub fn remove_allowed_app(&mut self, app_name: &str) {
        self.allowed_apps.retain(|app| app != app_name);
    }

    pub fn add_blocked_contact(&mut self, contact: String) {
        if !self.blocked_contacts.contains(&contact) {
            self.blocked_contacts.push(contact);
        }
    }

    pub fn remove_blocked_contact(&mut self, contact: &str) {
        self.blocked_contacts.retain(|c| c != contact);
    }

    pub fn set_schedule(&mut self, start: u32, end: u32) {
        if start < 1440 && end <= 1440 && start < end {
            self.schedule_start = start;
            self.schedule_end = end;
        }
    }

    pub fn is_within_schedule(&self, current_time: u32) -> bool {
        current_time >= self.schedule_start && current_time < self.schedule_end
    }

    pub fn should_block_notification(
        &self,
        app_name: &str,
        contact: Option<&str>,
        current_time: u32,
     -> bool {
        if !self.enabled || self.is_within_schedule(current_time) {
            return false;
        }
        if self.allowed_apps.contains(&app_name.to_string()) {
            return false;
        }
        if let Some(contact) = contact {
            if self.blocked_contacts.contains(&contact.to_string()) {
                return true;
            }
        }
        true
    }
}
