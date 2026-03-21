extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AIAgent {
    name: String,
    actions_approved: Vec<String>,
    actions_denied: Vec<String>,
}

impl AIAgent {
    pub fn new(name: &str) -> Self {
        AIAgent {
            name: String::from(name),
            actions_approved: Vec::new(),
            actions_denied: Vec::new(),
        }
    }

    pub fn approve_action(&mut self, action: &str) {
        self.actions_approved.push(String::from(action));
    }

    pub fn deny_action(&mut self, action: &str) {
        self.actions_denied.push(String::from(action));
    }

    pub fn get_approved_actions(&self) -> Vec<String> {
        self.actions_approved.clone()
    }

    pub fn get_denied_actions(&self) -> Vec<String> {
        self.actions_denied.clone()
    }

    pub fn has_approved_action(&self, action: &str) -> bool {
        self.actions_approved.contains(&String::from(action))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_agent() {
        let mut agent = AIAgent::new("Agent Smith");
        assert_eq!(agent.name, "Agent Smith");

        agent.approve_action("Monitor network traffic");
        agent.deny_action("Access restricted files");

        assert!(agent.has_approved_action("Monitor network traffic"));
        assert!(!agent.has_approved_action("Access restricted files"));

        let approved_actions = agent.get_approved_actions();
        let denied_actions = agent.get_denied_actions();

        assert_eq!(approved_actions, vec![String::from("Monitor network traffic")]);
        assert_eq!(denied_actions, vec![String::from("Access restricted files")]);
    }
}
