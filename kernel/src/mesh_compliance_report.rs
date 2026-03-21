extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MeshComplianceReport {
    node_id: String,
    compliance_status: bool,
    issues: Vec<String>,
    timestamp: u64,
}

impl MeshComplianceReport {
    pub fn new(node_id: &str, compliance_status: bool) -> Self {
        MeshComplianceReport {
            node_id: String::from(node_id),
            compliance_status,
            issues: Vec::new(),
            timestamp: 0, // Placeholder for actual timestamp logic
        }
    }

    pub fn add_issue(&mut self, issue: &str) {
        self.issues.push(String::from(issue));
    }

    pub fn get_node_id(&self) -> &String {
        &self.node_id
    }

    pub fn is_compliant(&self) -> bool {
        self.compliance_status
    }

    pub fn get_issues(&self) -> &Vec<String> {
        &self.issues
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
}
