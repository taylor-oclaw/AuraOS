extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

pub struct AuraAgentTemplate {
    name: String,
    version: u32,
    capabilities: Vec<String>,
    status: String,
    logs: Vec<String>,
}

impl AuraAgentTemplate {
    pub fn new(name: &str, version: u32) -> Self {
        AuraAgentTemplate {
            name: String::from(name),
            version,
            capabilities: Vec::new(),
            status: String::from("inactive"),
            logs: Vec::new(),
        }
    }

    pub fn add_capability(&mut self, capability: &str) {
        self.capabilities.push(String::from(capability));
    }

    pub fn activate(&mut self) {
        self.status = String::from("active");
        self.log_event("Agent activated");
    }

    pub fn deactivate(&mut self) {
        self.status = String::from("inactive");
        self.log_event("Agent deactivated");
    }

    pub fn log_event(&mut self, event: &str) {
        self.logs.push(String::from(event));
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_agent_template() {
        let mut agent = AuraAgentTemplate::new("TestAgent", 1);
        assert_eq!(agent.get_status(), "inactive");

        agent.add_capability("Capability1");
        agent.add_capability("Capability2");
        assert_eq!(agent.capabilities.len(), 2);

        agent.activate();
        assert_eq!(agent.get_status(), "active");

        agent.deactivate();
        assert_eq!(agent.get_status(), "inactive");

        assert_eq!(agent.logs.len(), 2);
    }
}
