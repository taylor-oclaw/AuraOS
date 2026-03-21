extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailContactEnrich {
    contacts: Vec<Contact>,
}

impl EmailContactEnrich {
    pub fn new() -> Self {
        EmailContactEnrich {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, email: String, name: Option<String>) {
        let contact = Contact { email, name };
        self.contacts.push(contact);
    }

    pub fn get_contact_by_email(&self, email: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.email == email)
    }

    pub fn remove_contact_by_email(&mut self, email: &str) {
        self.contacts.retain(|c| c.email != email);
    }

    pub fn list_contacts(&self) -> Vec<String> {
        self.contacts
            .iter()
            .map(|c| match &c.name {
                Some(name) => format!("{} <{}>", name, c.email),
                None => c.email.clone(),
            })
            .collect()
    }

    pub fn update_contact_name(&mut self, email: &str, new_name: String) -> bool {
        if let Some(contact) = self.contacts.iter_mut().find(|c| c.email == email) {
            contact.name = Some(new_name);
            true
        } else {
            false
        }
    }
}

struct Contact {
    email: String,
    name: Option<String>,
}
