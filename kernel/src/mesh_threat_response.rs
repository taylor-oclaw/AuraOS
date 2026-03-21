extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_threat_response_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_threat_response_exit() {
    // Cleanup logic for the module
}

pub struct ThreatResponse {
    threats: Vec<String>,
    responses: Vec<String>,
}

impl ThreatResponse {
    pub fn new() -> Self {
        ThreatResponse {
            threats: Vec::new(),
            responses: Vec::new(),
        }
    }

    pub fn add_threat(&mut self, threat: String) {
        self.threats.push(threat);
    }

    pub fn add_response(&mut self, response: String) {
        self.responses.push(response);
    }

    pub fn get_threat_count(&self) -> usize {
        self.threats.len()
    }

    pub fn get_response_count(&self) -> usize {
        self.responses.len()
    }

    pub fn generate_response(&self, threat_index: usize) -> Option<&String> {
        if threat_index < self.responses.len() {
            Some(&self.responses[threat_index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_response() {
        let mut tr = ThreatResponse::new();
        tr.add_threat(String::from("Malware"));
        tr.add_threat(String::from("Phishing"));
        tr.add_response(String::from("Run antivirus scan"));
        tr.add_response(String::from("Verify email sender"));

        assert_eq!(tr.get_threat_count(), 2);
        assert_eq!(tr.get_response_count(), 2);

        let response = tr.generate_response(0);
        assert_eq!(response, Some(&String::from("Run antivirus scan")));

        let response = tr.generate_response(1);
        assert_eq!(response, Some(&String::from("Verify email sender")));

        let response = tr.generate_response(2);
        assert_eq!(response, None);
    }
}
