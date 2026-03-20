extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentCapabilityDiscovery {
    capabilities: Vec<String>,
}

impl AgentCapabilityDiscovery {
    pub fn new() -> Self {
        AgentCapabilityDiscovery {
            capabilities: Vec::new(),
        }
    }

    pub fn add_capability(&mut self, capability: String) {
        self.capabilities.push(capability);
    }

    pub fn remove_capability(&mut self, capability: &str) -> bool {
        if let Some(index) = self.capabilities.iter().position(|c| c == capability) {
            self.capabilities.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&String::from(capability))
    }

    pub fn list_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }

    pub fn count_capabilities(&self) -> usize {
        self.capabilities.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_capability_discovery() {
        let mut acd = AgentCapabilityDiscovery::new();
        assert_eq!(acd.count_capabilities(), 0);

        acd.add_capability(String::from("AI"));
        acd.add_capability(String::from("Machine Learning"));
        assert_eq!(acd.count_capabilities(), 2);
        assert!(acd.has_capability("AI"));
        assert!(!acd.has_capability("Blockchain"));

        let capabilities = acd.list_capabilities();
        assert_eq!(capabilities.len(), 2);
        assert!(capabilities.contains(&String::from("AI")));
        assert!(capabilities.contains(&String::from("Machine Learning")));

        assert!(acd.remove_capability("AI"));
        assert!(!acd.has_capability("AI"));
        assert_eq!(acd.count_capabilities(), 1);

        assert!(!acd.remove_capability("Blockchain"));
        assert_eq!(acd.count_capabilities(), 1);
    }
}
