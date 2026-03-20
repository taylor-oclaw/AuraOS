extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentRuleEngine {
    rules: Vec<String>,
}

impl AgentRuleEngine {
    pub fn new() -> Self {
        AgentRuleEngine { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, index: usize) -> Option<String> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    pub fn get_rules(&self) -> &Vec<String> {
        &self.rules
    }

    pub fn has_rule(&self, rule: &str) -> bool {
        self.rules.contains(&String::from(rule))
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_rule_engine() {
        let mut engine = AgentRuleEngine::new();

        assert_eq!(engine.get_rules().len(), 0);

        engine.add_rule(String::from("rule1"));
        engine.add_rule(String::from("rule2"));

        assert_eq!(engine.get_rules().len(), 2);
        assert!(engine.has_rule("rule1"));
        assert!(!engine.has_rule("rule3"));

        let removed = engine.remove_rule(0);
        assert_eq!(removed, Some(String::from("rule1")));
        assert_eq!(engine.get_rules().len(), 1);

        engine.clear_rules();
        assert_eq!(engine.get_rules().len(), 0);
    }
}
