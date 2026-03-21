extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyHubEmergencyContact {
    name: String,
    phone_number: String,
    relationship: String,
    address: String,
    email: String,
}

impl FamilyHubEmergencyContact {
    pub fn new(name: &str, phone_number: &str, relationship: &str, address: &str, email: &str) -> Self {
        FamilyHubEmergencyContact {
            name: String::from(name),
            phone_number: String::from(phone_number),
            relationship: String::from(relationship),
            address: String::from(address),
            email: String::from(email),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn get_relationship(&self) -> &str {
        &self.relationship
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}

pub struct EmergencyContactList {
    contacts: Vec<FamilyHubEmergencyContact>,
}

impl EmergencyContactList {
    pub fn new() -> Self {
        EmergencyContactList {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: FamilyHubEmergencyContact) {
        self.contacts.push(contact);
    }

    pub fn remove_contact_by_name(&mut self, name: &str) {
        self.contacts.retain(|c| c.get_name() != name);
    }

    pub fn get_contact_by_name(&self, name: &str) -> Option<&FamilyHubEmergencyContact> {
        self.contacts.iter().find(|c| c.get_name() == name)
    }

    pub fn list_contacts(&self) -> &[FamilyHubEmergencyContact] {
        &self.contacts
    }
}
