extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_contact_cleanup_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_contact_cleanup_exit() {
    // Cleanup logic for the module
}

pub struct ContactCleanup {
    contacts: Vec<String>,
}

impl ContactCleanup {
    pub fn new() -> Self {
        ContactCleanup {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: String) {
        self.contacts.push(contact);
    }

    pub fn remove_contact(&mut self, contact: &str) -> bool {
        if let Some(index) = self.contacts.iter().position(|c| c == contact) {
            self.contacts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_contacts(&self) -> &[String] {
        &self.contacts
    }

    pub fn clear_contacts(&mut self) {
        self.contacts.clear();
    }

    pub fn has_contact(&self, contact: &str) -> bool {
        self.contacts.contains(contact)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_contact() {
        let mut cleanup = ContactCleanup::new();
        assert_eq!(cleanup.get_contacts().len(), 0);

        cleanup.add_contact(String::from("Alice"));
        assert_eq!(cleanup.get_contacts().len(), 1);
        assert!(cleanup.has_contact("Alice"));

        assert!(cleanup.remove_contact("Alice"));
        assert!(!cleanup.has_contact("Alice"));
        assert_eq!(cleanup.get_contacts().len(), 0);
    }

    #[test]
    fn test_clear_contacts() {
        let mut cleanup = ContactCleanup::new();
        cleanup.add_contact(String::from("Bob"));
        cleanup.add_contact(String::from("Charlie"));

        assert_eq!(cleanup.get_contacts().len(), 2);

        cleanup.clear_contacts();
        assert_eq!(cleanup.get_contacts().len(), 0);
    }
}
