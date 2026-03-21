extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct Contact {
    name: String,
    email: String,
}

impl Contact {
    pub fn new(name: &str, email: &str) -> Self {
        Contact {
            name: String::from(name),
            email: String::from(email),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = String::from(email);
    }
}

struct ContactDeduplicator {
    contacts: Vec<Contact>,
}

impl ContactDeduplicator {
    pub fn new() -> Self {
        ContactDeduplicator {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, name: &str, email: &str) {
        if !self.contacts.iter().any(|c| c.email == email) {
            self.contacts.push(Contact::new(name, email));
        }
    }

    pub fn remove_contact_by_email(&mut self, email: &str) {
        self.contacts.retain(|c| c.email != email);
    }

    pub fn get_contacts(&self) -> &[Contact] {
        &self.contacts
    }

    pub fn find_contact_by_email(&self, email: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.email == email)
    }
}
