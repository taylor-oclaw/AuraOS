extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RelationshipAnniversaryRemind {
    reminders: Vec<(String, u32)>, // (partner's name, days until anniversary)
}

impl RelationshipAnniversaryRemind {
    pub fn new() -> Self {
        RelationshipAnniversaryRemind {
            reminders: Vec::new(),
        }
    }

    pub fn add_reminder(&mut self, partner_name: String, days_until_anniversary: u32) {
        self.reminders.push((partner_name, days_until_anniversary));
    }

    pub fn remove_reminder(&mut self, partner_name: &str) -> bool {
        let pos = self
            .reminders
            .iter()
            .position(|&(ref name, _)| name == partner_name);
        if let Some(index) = pos {
            self.reminders.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_reminder(&self, partner_name: &str) -> Option<&(String, u32)> {
        self.reminders.iter().find(|&&(ref name, _)| name == partner_name)
    }

    pub fn list_all_reminders(&self) -> Vec<&(String, u32)> {
        self.reminders.iter().collect()
    }

    pub fn update_days_until_anniversary(
        &mut self,
        partner_name: &str,
        new_days: u32,
     -> bool {
        if let Some((_, days)) = self
            .reminders
            .iter_mut()
            .find(|(ref name, _)| name == partner_name)
        {
            *days = new_days;
            true
        } else {
            false
        }
    }
}
