extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_example() -> i32 {
    42
}

struct AuraAgentBuilder {
    name: String,
    capabilities: Vec<String>,
    version: u32,
    active: bool,
    configuration: String,
}

impl AuraAgentBuilder {
    pub fn new(name: &str, version: u32) -> Self {
        AuraAgentBuilder {
            name: String::from(name),
            capabilities: Vec::new(),
            version,
            active: false,
            configuration: String::new(),
        }
    }

    pub fn add_capability(&mut self, capability: &str) {
        self.capabilities.push(String::from(capability));
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_agent_builder() {
        let mut agent = AuraAgentBuilder::new("TestAgent", 1);
        assert_eq!(agent.get_name(), "TestAgent");
        assert_eq!(agent.get_version(), 1);
        assert!(!agent.is_active());

        agent.add_capability("Capability1");
        agent.add_capability("Capability2");
        assert_eq!(agent.capabilities.len(), 2);

        agent.set_active(true);
        assert!(agent.is_active());
    }
}
