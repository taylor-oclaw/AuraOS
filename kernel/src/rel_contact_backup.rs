extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ContactBackup {
    contacts: Vec<String>,
}

impl ContactBackup {
    pub fn new() -> Self {
        ContactBackup {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: String) {
        if !self.contacts.contains(&contact) {
            self.contacts.push(contact);
        }
    }

    pub fn remove_contact(&mut self, contact: &str) -> bool {
        let position = self.contacts.iter().position(|c| c == contact);
        match position {
            Some(index) => {
                self.contacts.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn get_contacts(&self) -> &[String] {
        &self.contacts
    }

    pub fn find_contact(&self, contact: &str) -> Option<&String> {
        self.contacts.iter().find(|c| c == &contact)
    }

    pub fn clear_contacts(&mut self) {
        self.contacts.clear();
    }
}
