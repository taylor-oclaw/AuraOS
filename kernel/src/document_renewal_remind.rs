extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentRenewalRemind {
    reminders: Vec<(String, u32)>, // (document_name, days_until_renewal)
}

impl DocumentRenewalRemind {
    pub fn new() -> Self {
        DocumentRenewalRemind {
            reminders: Vec::new(),
        }
    }

    pub fn add_reminder(&mut self, document_name: String, days_until_renewal: u32) {
        self.reminders.push((document_name, days_until_renewal));
    }

    pub fn remove_reminder(&mut self, document_name: &str) -> bool {
        let position = self
            .reminders
            .iter()
            .position(|(name, _)| name == document_name);
        if let Some(pos) = position {
            self.reminders.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_reminder(&self, document_name: &str) -> Option<&(String, u32)> {
        self.reminders.iter().find(|&&(name, _)| name == document_name)
    }

    pub fn list_all_reminders(&self) -> Vec<(String, u32)> {
        self.reminders.clone()
    }

    pub fn update_days_until_renewal(&mut self, document_name: &str, new_days: u32) -> bool {
        if let Some((_, days)) = self
            .reminders
            .iter_mut()
            .find(|(name, _)| name == document_name)
        {
            *days = new_days;
            true
        } else {
            false
        }
    }
}
