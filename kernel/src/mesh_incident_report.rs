extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshIncidentReport {
    incidents: Vec<String>,
}

impl MeshIncidentReport {
    pub fn new() -> Self {
        MeshIncidentReport {
            incidents: Vec::new(),
        }
    }

    pub fn add_incident(&mut self, incident: &str) {
        self.incidents.push(String::from(incident));
    }

    pub fn get_incidents(&self) -> &[String] {
        &self.incidents
    }

    pub fn clear_incidents(&mut self) {
        self.incidents.clear();
    }

    pub fn has_incident(&self, incident: &str) -> bool {
        self.incidents.iter().any(|i| i == incident)
    }

    pub fn count_incidents(&self) -> usize {
        self.incidents.len()
    }
}
