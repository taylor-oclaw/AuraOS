extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AgentToolRegistry {
    tools: Vec<String>,
}

impl AgentToolRegistry {
    pub fn new() -> Self {
        AgentToolRegistry { tools: Vec::new() }
    }

    pub fn add_tool(&mut self, tool_name: &str) {
        self.tools.push(String::from(tool_name));
    }

    pub fn remove_tool(&mut self, tool_name: &str) {
        if let Some(index) = self.tools.iter().position(|t| t == tool_name) {
            self.tools.remove(index);
        }
    }

    pub fn has_tool(&self, tool_name: &str) -> bool {
        self.tools.contains(&String::from(tool_name))
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.clone()
    }

    pub fn count_tools(&self) -> usize {
        self.tools.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_tool_registry() {
        let mut registry = AgentToolRegistry::new();
        assert_eq!(registry.count_tools(), 0);

        registry.add_tool("tool1");
        registry.add_tool("tool2");
        assert_eq!(registry.count_tools(), 2);
        assert!(registry.has_tool("tool1"));
        assert!(registry.has_tool("tool2"));

        let tools = registry.list_tools();
        assert_eq!(tools.len(), 2);
        assert!(tools.contains(&String::from("tool1")));
        assert!(tools.contains(&String::from("tool2")));

        registry.remove_tool("tool1");
        assert!(!registry.has_tool("tool1"));
        assert_eq!(registry.count_tools(), 1);

        registry.remove_tool("tool2");
        assert_eq!(registry.count_tools(), 0);
    }
}
