extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct Contact {
    name: String,
    email: String,
    phone: String,
    address: String,
    notes: Vec<String>,
}

impl Contact {
    pub fn new(name: &str, email: &str, phone: &str, address: &str) -> Self {
        Contact {
            name: String::from(name),
            email: String::from(email),
            phone: String::from(phone),
            address: String::from(address),
            notes: Vec::new(),
        }
    }

    pub fn add_note(&mut self, note: &str) {
        self.notes.push(String::from(note));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_phone(&self) -> &str {
        &self.phone
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_notes(&self) -> &[String] {
        &self.notes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_creation() {
        let contact = Contact::new("John Doe", "john@example.com", "123-456-7890", "123 Main St");
        assert_eq!(contact.get_name(), "John Doe");
        assert_eq!(contact.get_email(), "john@example.com");
        assert_eq!(contact.get_phone(), "123-456-7890");
        assert_eq!(contact.get_address(), "123 Main St");
    }

    #[test]
    fn test_add_note() {
        let mut contact = Contact::new("John Doe", "john@example.com", "123-456-7890", "123 Main St");
        contact.add_note("Met at conference.");
        assert_eq!(contact.get_notes().len(), 1);
        assert_eq!(contact.get_notes()[0], "Met at conference.");
    }
}
