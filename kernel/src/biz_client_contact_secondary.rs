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

struct ContactList {
    contacts: Vec<Contact>,
}

impl ContactList {
    pub fn new() -> Self {
        ContactList {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    pub fn remove_contact_by_name(&mut self, name: &str) {
        self.contacts.retain(|c| c.get_name() != name);
    }

    pub fn get_contacts(&self) -> &[Contact] {
        &self.contacts
    }

    pub fn find_contact_by_name(&self, name: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.get_name() == name)
    }
}
