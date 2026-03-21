extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct IncidentReport {
    id: u32,
    description: String,
    severity: Severity,
    timestamp: u64,
    evidence: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum Severity {
    Low,
    Medium,
    High,
}

impl IncidentReport {
    pub fn new(id: u32, description: &str, severity: Severity, timestamp: u64) -> Self {
        IncidentReport {
            id,
            description: String::from(description),
            severity,
            timestamp,
            evidence: Vec::new(),
        }
    }

    pub fn add_evidence(&mut self, evidence_item: &str) {
        self.evidence.push(String::from(evidence_item));
    }

    pub fn get_severity(&self) -> Severity {
        self.severity
    }

    pub fn set_severity(&mut self, severity: Severity) {
        self.severity = severity;
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn update_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incident_report() {
        let mut report = IncidentReport::new(1, "Unauthorized access attempt", Severity::Medium, 1633072800);

        assert_eq!(report.get_severity(), Severity::Medium);
        assert_eq!(report.get_description(), "Unauthorized access attempt");

        report.set_severity(Severity::High);
        report.update_description("Critical unauthorized access detected");
        report.add_evidence("Log entry 12345");
        report.add_evidence("IP address: 192.168.1.1");

        assert_eq!(report.get_severity(), Severity::High);
        assert_eq!(report.get_description(), "Critical unauthorized access detected");
        assert_eq!(report.evidence.len(), 2);
    }
}
