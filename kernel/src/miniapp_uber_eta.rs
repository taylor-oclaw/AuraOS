extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let uber_eta = MiniAppUberEta::new();
    uber_eta.update_eta("45 minutes");
    uber_eta.set_destination("123 Main St");
    uber_eta.add_note(String::from("Avoid rush hour"));
    uber_eta.clear_notes();
}

pub struct MiniAppUberEta {
    eta: String,
    destination: String,
    notes: Vec<String>,
}

impl MiniAppUberEta {
    pub fn new() -> Self {
        MiniAppUberEta {
            eta: String::from("N/A"),
            destination: String::from("Unknown"),
            notes: Vec::new(),
        }
    }

    pub fn update_eta(&mut self, eta: &str) {
        self.eta = eta.to_string();
    }

    pub fn get_eta(&self) -> &str {
        &self.eta
    }

    pub fn set_destination(&mut self, destination: &str) {
        self.destination = destination.to_string();
    }

    pub fn get_destination(&self) -> &str {
        &self.destination
    }

    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    pub fn get_notes(&self) -> &[String] {
        &self.notes
    }

    pub fn clear_notes(&mut self) {
        self.notes.clear();
    }
}
