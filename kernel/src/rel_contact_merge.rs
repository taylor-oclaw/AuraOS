extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct Contact {
    name: String,
    email: String,
    phone: String,
}

impl Contact {
    pub fn new(name: &str, email: &str, phone: &str) -> Self {
        Contact {
            name: String::from(name),
            email: String::from(email),
            phone: String::from(phone),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = String::from(email);
    }

    pub fn get_phone(&self) -> &str {
        &self.phone
    }

    pub fn set_phone(&mut self, phone: &str) {
        self.phone = String::from(phone);
    }
}

pub struct ContactManager {
    contacts: Vec<Contact>,
}

impl ContactManager {
    pub fn new() -> Self {
        ContactManager {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    pub fn remove_contact_by_name(&mut self, name: &str) {
        self.contacts.retain(|c| c.get_name() != name);
    }

    pub fn find_contact_by_name(&self, name: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.get_name() == name)
    }

    pub fn list_contacts(&self) -> Vec<String> {
        self.contacts
            .iter()
            .map(|c| format!("Name: {}, Email: {}, Phone: {}", c.get_name(), c.get_email(), c.get_phone()))
            .collect()
    }
}
