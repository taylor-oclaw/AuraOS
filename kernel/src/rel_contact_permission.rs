extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_contact_permission_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_contact_permission_exit() {
    // Cleanup logic for the module
}

pub struct ContactPermission {
    contacts: Vec<String>,
    permissions: Vec<bool>,
}

impl ContactPermission {
    pub fn new() -> Self {
        ContactPermission {
            contacts: Vec::new(),
            permissions: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: String) {
        if !self.contacts.contains(&contact) {
            self.contacts.push(contact);
            self.permissions.push(false); // Default permission is false
        }
    }

    pub fn remove_contact(&mut self, contact: &str) -> bool {
        if let Some(index) = self.contacts.iter().position(|c| c == contact) {
            self.contacts.remove(index);
            self.permissions.remove(index);
            true
        } else {
            false
        }
    }

    pub fn grant_permission(&mut self, contact: &str) -> bool {
        if let Some(index) = self.contacts.iter().position(|c| c == contact) {
            self.permissions[index] = true;
            true
        } else {
            false
        }
    }

    pub fn revoke_permission(&mut self, contact: &str) -> bool {
        if let Some(index) = self.contacts.iter().position(|c| c == contact) {
            self.permissions[index] = false;
            true
        } else {
            false
        }
    }

    pub fn check_permission(&self, contact: &str) -> Option<bool> {
        self.contacts.iter().position(|c| c == contact).map(|index| self.permissions[index])
    }
}
