extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod agent_attention_system {
    use super::*;

    pub struct AgentAttentionSystem {
        agents: Vec<String>,
        attention_levels: Vec<u8>,
    }

    impl AgentAttentionSystem {
        pub fn new() -> Self {
            AgentAttentionSystem {
                agents: Vec::new(),
                attention_levels: Vec::new(),
            }
        }

        pub fn add_agent(&mut self, agent_name: &str) {
            if !self.agents.contains(&agent_name.to_string()) {
                self.agents.push(agent_name.to_string());
                self.attention_levels.push(0);
            }
        }

        pub fn remove_agent(&mut self, agent_name: &str) {
            if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
                self.agents.remove(index);
                self.attention_levels.remove(index);
            }
        }

        pub fn set_attention_level(&mut self, agent_name: &str, level: u8) {
            if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
                self.attention_levels[index] = level;
            }
        }

        pub fn get_attention_level(&self, agent_name: &str) -> Option<u8> {
            self.agents.iter().position(|a| a == agent_name).map(|index| self.attention_levels[index])
        }

        pub fn list_agents(&self) -> Vec<&String> {
            self.agents.iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::agent_attention_system::*;

    #[test]
    fn test_agent_attention_system() {
        let mut system = AgentAttentionSystem::new();

        // Add agents
        system.add_agent("Agent1");
        system.add_agent("Agent2");

        // Check if agents are added correctly
        assert_eq!(system.list_agents(), vec!["Agent1", "Agent2"]);

        // Set attention levels
        system.set_attention_level("Agent1", 5);
        system.set_attention_level("Agent2", 3);

        // Check attention levels
        assert_eq!(system.get_attention_level("Agent1"), Some(5));
        assert_eq!(system.get_attention_level("Agent2"), Some(3));

        // Remove an agent
        system.remove_agent("Agent1");

        // Check if agent is removed correctly
        assert!(!system.list_agents().contains(&String::from("Agent1")));
    }
}
