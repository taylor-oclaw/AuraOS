extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_contact_share_family_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_contact_share_family_exit() {
    // Cleanup logic for the module
}

pub struct Contact {
    name: String,
    phone_number: String,
    email: String,
}

impl Contact {
    pub fn new(name: &str, phone_number: &str, email: &str) -> Self {
        Contact {
            name: String::from(name),
            phone_number: String::from(phone_number),
            email: String::from(email),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn update_phone_number(&mut self, new_phone: &str) {
        self.phone_number = String::from(new_phone);
    }

    pub fn update_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

pub struct FamilyContacts {
    contacts: Vec<Contact>,
}

impl FamilyContacts {
    pub fn new() -> Self {
        FamilyContacts {
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
