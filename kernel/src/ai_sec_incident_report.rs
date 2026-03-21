extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

pub struct AISecIncidentReport {
    incidents: Vec<String>,
}

impl AISecIncidentReport {
    pub fn new() -> Self {
        AISecIncidentReport {
            incidents: Vec::new(),
        }
    }

    pub fn add_incident(&mut self, incident: &str) {
        self.incidents.push(String::from(incident));
    }

    pub fn get_incident_count(&self) -> usize {
        self.incidents.len()
    }

    pub fn get_all_incidents(&self) -> Vec<String> {
        self.incidents.clone()
    }

    pub fn clear_incidents(&mut self) {
        self.incidents.clear();
    }

    pub fn has_incident(&self, incident: &str) -> bool {
        self.incidents.contains(&String::from(incident))
    }
}
