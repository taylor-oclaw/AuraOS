extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct BizClientContactBlocker {
    blocked_contacts: Vec<String>,
}

impl BizClientContactBlocker {
    pub fn new() -> Self {
        BizClientContactBlocker {
            blocked_contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: String) {
        if !self.blocked_contacts.contains(&contact) {
            self.blocked_contacts.push(contact);
        }
    }

    pub fn remove_contact(&mut self, contact: &str) -> bool {
        let pos = self.blocked_contacts.iter().position(|c| c == contact);
        match pos {
            Some(index) => {
                self.blocked_contacts.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn is_contact_blocked(&self, contact: &str) -> bool {
        self.blocked_contacts.contains(contact)
    }

    pub fn list_blocked_contacts(&self) -> Vec<String> {
        self.blocked_contacts.clone()
    }

    pub fn clear_all_contacts(&mut self) {
        self.blocked_contacts.clear();
    }
}
