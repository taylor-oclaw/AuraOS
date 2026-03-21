extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RelationshipContactEnrich {
    contacts: Vec<Contact>,
}

impl RelationshipContactEnrich {
    pub fn new() -> Self {
        RelationshipContactEnrich {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, name: String, phone: String) {
        let contact = Contact { name, phone };
        self.contacts.push(contact);
    }

    pub fn get_contacts(&self) -> &Vec<Contact> {
        &self.contacts
    }

    pub fn find_contact_by_name(&self, name: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.name == name)
    }

    pub fn remove_contact_by_name(&mut self, name: &str) {
        self.contacts.retain(|c| c.name != name);
    }

    pub fn update_contact_phone(&mut self, name: &str, new_phone: String) -> bool {
        if let Some(contact) = self.contacts.iter_mut().find(|c| c.name == name) {
            contact.phone = new_phone;
            true
        } else {
            false
        }
    }
}

struct Contact {
    name: String,
    phone: String,
}
