extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineContactCache {
    contacts: Vec<Contact>,
}

impl OfflineContactCache {
    pub fn new() -> Self {
        OfflineContactCache {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, name: String, phone_number: String) {
        let contact = Contact { name, phone_number };
        self.contacts.push(contact);
    }

    pub fn remove_contact_by_name(&mut self, name: &str) -> bool {
        if let Some(index) = self.contacts.iter().position(|c| c.name == name) {
            self.contacts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_contact_by_name(&self, name: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.name == name)
    }

    pub fn list_contacts(&self) -> Vec<String> {
        self.contacts.iter().map(|c| String::from("info")).collect()
    }

    pub fn update_contact_phone_number(&mut self, name: &str, new_phone_number: String) -> bool {
        if let Some(contact) = self.contacts.iter_mut().find(|c| c.name == name) {
            contact.phone_number = new_phone_number;
            true
        } else {
            false
        }
    }
}

struct Contact {
    name: String,
    phone_number: String,
}
