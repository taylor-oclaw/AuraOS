extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

mod ai_sec_incident_response {
    use super::*;

    pub struct IncidentResponse {
        incidents: Vec<String>,
        response_plan: String,
    }

    impl IncidentResponse {
        pub fn new(response_plan: &str) -> Self {
            IncidentResponse {
                incidents: Vec::new(),
                response_plan: String::from(response_plan),
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

        pub fn get_response_plan(&self) -> &str {
            &self.response_plan
        }

        pub fn update_response_plan(&mut self, new_plan: &str) {
            self.response_plan = String::from(new_plan);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incident_response() {
        let mut ir = ai_sec_incident_response::IncidentResponse::new("Initial response plan");

        assert_eq!(ir.get_incidents().len(), 0);
        assert_eq!(ir.get_response_plan(), "Initial response plan");

        ir.add_incident("Malware detected");
        ir.add_incident("Unauthorized access attempt");

        assert_eq!(ir.get_incidents().len(), 2);
        assert_eq!(ir.get_incidents()[0], "Malware detected");
        assert_eq!(ir.get_incidents()[1], "Unauthorized access attempt");

        ir.clear_incidents();
        assert_eq!(ir.get_incidents().len(), 0);

        ir.update_response_plan("Updated response plan");
        assert_eq!(ir.get_response_plan(), "Updated response plan");
    }
}
