extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AISecurityIncidentContain {
    incidents: Vec<String>,
    containment_actions: Vec<String>,
}

impl AISecurityIncidentContain {
    pub fn new() -> Self {
        AISecurityIncidentContain {
            incidents: Vec::new(),
            containment_actions: Vec::new(),
        }
    }

    pub fn report_incident(&mut self, incident_description: &str) {
        let description = String::from(incident_description);
        self.incidents.push(description);
    }

    pub fn get_incidents(&self) -> &[String] {
        &self.incidents
    }

    pub fn take_containment_action(&mut self, action_description: &str) {
        let action = String::from(action_description);
        self.containment_actions.push(action);
    }

    pub fn get_containment_actions(&self) -> &[String] {
        &self.containment_actions
    }

    pub fn clear_incidents(&mut self) {
        self.incidents.clear();
    }
}
