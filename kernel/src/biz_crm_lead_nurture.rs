extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct Lead {
    name: String,
    email: String,
    phone: String,
    status: String,
    notes: Vec<String>,
}

impl Lead {
    pub fn new(name: &str, email: &str, phone: &str) -> Self {
        Lead {
            name: String::from(name),
            email: String::from(email),
            phone: String::from(phone),
            status: String::from("New"),
            notes: Vec::new(),
        }
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
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

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_notes(&self) -> &[String] {
        &self.notes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lead_creation() {
        let lead = Lead::new("John Doe", "john@example.com", "123-456-7890");
        assert_eq!(lead.get_name(), "John Doe");
        assert_eq!(lead.get_email(), "john@example.com");
        assert_eq!(lead.get_phone(), "123-456-7890");
        assert_eq!(lead.get_status(), "New");
        assert!(lead.get_notes().is_empty());
    }

    #[test]
    fn test_lead_update_status() {
        let mut lead = Lead::new("John Doe", "john@example.com", "123-456-7890");
        lead.update_status("Converted");
        assert_eq!(lead.get_status(), "Converted");
    }

    #[test]
    fn test_lead_add_note() {
        let mut lead = Lead::new("John Doe", "john@example.com", "123-456-7890");
        lead.add_note("Initial contact made.");
        assert_eq!(lead.get_notes().len(), 1);
        assert_eq!(lead.get_notes()[0], "Initial contact made.");
    }
}
