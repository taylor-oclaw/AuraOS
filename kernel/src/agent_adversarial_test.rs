extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentAdversarialTest {
    name: String,
    actions: Vec<String>,
    status: String,
}

impl AgentAdversarialTest {
    pub fn new(name: &str) -> Self {
        AgentAdversarialTest {
            name: String::from(name),
            actions: Vec::new(),
            status: String::from("Idle"),
        }
    }

    pub fn add_action(&mut self, action: &str) {
        self.actions.push(String::from(action));
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_actions(&self) -> &Vec<String> {
        &self.actions
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
    }

    pub fn get_status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = AgentAdversarialTest::new("TestAgent");
        assert_eq!(agent.get_name(), "TestAgent");
        assert_eq!(agent.get_status(), "Idle");
        assert!(agent.get_actions().is_empty());
    }

    #[test]
    fn test_add_action() {
        let mut agent = AgentAdversarialTest::new("TestAgent");
        agent.add_action("Attack");
        agent.add_action("Defend");
        assert_eq!(agent.get_actions(), &vec![String::from("Attack"), String::from("Defend")]);
    }

    #[test]
    fn test_set_status() {
        let mut agent = AgentAdversarialTest::new("TestAgent");
        agent.set_status("Active");
        assert_eq!(agent.get_status(), "Active");
    }
}
