extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn relationship_birthday_remind_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn relationship_birthday_remind_exit() {
    // Cleanup logic for the module
}

pub struct RelationshipBirthdayRemind {
    relationships: Vec<(String, u32)>, // (name, days_until_birthday)
}

impl RelationshipBirthdayRemind {
    pub fn new() -> Self {
        RelationshipBirthdayRemind {
            relationships: Vec::new(),
        }
    }

    pub fn add_relationship(&mut self, name: String, days_until_birthday: u32) {
        self.relationships.push((name, days_until_birthday));
    }

    pub fn remove_relationship(&mut self, name: &str) -> bool {
        let pos = self.relationships.iter().position(|(n, _)| n == name);
        if let Some(index) = pos {
            self.relationships.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_relationships(&self) -> &Vec<(String, u32)> {
        &self.relationships
    }

    pub fn update_birthday(&mut self, name: &str, new_days_until_birthday: u32) -> bool {
        if let Some((_, days)) = self.relationships.iter_mut().find(|(n, _)| n == name) {
            *days = new_days_until_birthday;
            true
        } else {
            false
        }
    }

    pub fn upcoming_birthdays(&self, max_days: u32) -> Vec<&String> {
        self.relationships
            .iter()
            .filter(|&(_, days)| *days <= max_days)
            .map(|(name, _)| name)
            .collect()
    }
}
