extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let lead = LeadQualify::new("John Doe", "john.doe@example.com");
    lead.add_note(String::from("Initial contact made"));
    lead.update_status(LeadStatus::Qualified);
    lead.add_phone_number(String::from("+1234567890"));
    lead.add_email(String::from("johndoe@newemail.com"));

    loop {}
}

#[derive(Debug, PartialEq)]
enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Lost,
    Closed,
}

pub struct LeadQualify {
    name: String,
    email: String,
    phone_numbers: Vec<String>,
    emails: Vec<String>,
    notes: Vec<String>,
    status: LeadStatus,
}

impl LeadQualify {
    pub fn new(name: &str, email: &str) -> Self {
        LeadQualify {
            name: String::from(name),
            email: String::from(email),
            phone_numbers: Vec::new(),
            emails: Vec::new(),
            notes: Vec::new(),
            status: LeadStatus::New,
        }
    }

    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    pub fn update_status(&mut self, status: LeadStatus) {
        self.status = status;
    }

    pub fn add_phone_number(&mut self, phone: String) {
        self.phone_numbers.push(phone);
    }

    pub fn add_email(&mut self, email: String) {
        self.emails.push(email);
    }

    pub fn get_status(&self) -> LeadStatus {
        self.status
    }
}
