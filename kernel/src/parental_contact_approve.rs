extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn parental_contact_approve_init() {
    // Initialization logic for the module
}

pub extern "C" fn parental_contact_approve_exit() {
    // Cleanup logic for the module
}

pub struct ParentalContactApprover {
    approved_contacts: Vec<String>,
    blocked_contacts: Vec<String>,
}

impl ParentalContactApprover {
    pub fn new() -> Self {
        ParentalContactApprover {
            approved_contacts: Vec::new(),
            blocked_contacts: Vec::new(),
        }
    }

    pub fn add_approved_contact(&mut self, contact: String) {
        if !self.approved_contacts.contains(&contact) && !self.blocked_contacts.contains(&contact) {
            self.approved_contacts.push(contact);
        }
    }

    pub fn remove_approved_contact(&mut self, contact: &str) -> bool {
        if let Some(index) = self.approved_contacts.iter().position(|c| c == contact) {
            self.approved_contacts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_blocked_contact(&mut self, contact: String) {
        if !self.blocked_contacts.contains(&contact) && !self.approved_contacts.contains(&contact) {
            self.blocked_contacts.push(contact);
        }
    }

    pub fn remove_blocked_contact(&mut self, contact: &str) -> bool {
        if let Some(index) = self.blocked_contacts.iter().position(|c| c == contact) {
            self.blocked_contacts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_contact_approved(&self, contact: &str) -> bool {
        self.approved_contacts.contains(contact)
    }

    pub fn is_contact_blocked(&self, contact: &str) -> bool {
        self.blocked_contacts.contains(contact)
    }
}
