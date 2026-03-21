extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ContactFrequencyOptimal {
    contacts: Vec<String>,
    frequencies: Vec<u32>,
}

impl ContactFrequencyOptimal {
    pub fn new() -> Self {
        ContactFrequencyOptimal {
            contacts: Vec::new(),
            frequencies: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: String) {
        if !self.contacts.contains(&contact) {
            self.contacts.push(contact);
            self.frequencies.push(0);
        }
    }

    pub fn update_frequency(&mut self, contact: &str) {
        if let Some(index) = self.contacts.iter().position(|c| c == contact) {
            self.frequencies[index] += 1;
        }
    }

    pub fn get_contact_frequency(&self, contact: &str) -> Option<u32> {
        self.contacts.iter().position(|c| c == contact).map(|index| self.frequencies[index])
    }

    pub fn most_frequent_contact(&self) -> Option<&String> {
        self.contacts.iter().max_by_key(|&contact| {
            if let Some(index) = self.contacts.iter().position(|c| c == contact) {
                self.frequencies[index]
            } else {
                0
            }
        })
    }

    pub fn least_frequent_contact(&self) -> Option<&String> {
        self.contacts.iter().min_by_key(|&contact| {
            if let Some(index) = self.contacts.iter().position(|c| c == contact) {
                self.frequencies[index]
            } else {
                0
            }
        })
    }
}
