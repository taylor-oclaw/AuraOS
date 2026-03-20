extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AuraAgentConfig {
    pub agent_id: String,
    pub capabilities: Vec<String>,
    pub active: bool,
    pub priority: u8,
    pub last_updated: u64,
}

impl AuraAgentConfig {
    pub fn new(agent_id: &str, capabilities: &[&str], active: bool, priority: u8, last_updated: u64) -> Self {
        AuraAgentConfig {
            agent_id: String::from(agent_id),
            capabilities: capabilities.iter().map(|&cap| String::from(cap)).collect(),
            active,
            priority,
            last_updated,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&String::from(capability)) {
            self.capabilities.push(String::from(capability));
        }
    }

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|cap| cap != capability);
    }

    pub fn update_priority(&mut self, new_priority: u8) {
        self.priority = new_priority;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_agent_config() {
        let mut config = AuraAgentConfig::new("agent1", &["cap1", "cap2"], false, 50, 1633072800);
        
        assert_eq!(config.agent_id, "agent1");
        assert_eq!(config.capabilities.len(), 2);
        assert!(!config.active);
        assert_eq!(config.priority, 50);
        assert_eq!(config.last_updated, 1633072800);

        config.activate();
        assert!(config.active);

        config.deactivate();
        assert!(!config.active);

        config.add_capability("cap3");
        assert_eq!(config.capabilities.len(), 3);
        assert!(config.capabilities.contains(&String::from("cap3")));

        config.remove_capability("cap2");
        assert_eq!(config.capabilities.len(), 2);
        assert!(!config.capabilities.contains(&String::from("cap2")));

        config.update_priority(75);
        assert_eq!(config.priority, 75);
    }
}
