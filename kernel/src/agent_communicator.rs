extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentCommunicator {
    messages: Vec<String>,
    agent_id: String,
}

impl AgentCommunicator {
    pub fn new(agent_id: &str) -> Self {
        AgentCommunicator {
            messages: Vec::new(),
            agent_id: String::from(agent_id),
        }
    }

    pub fn add_message(&mut self, message: &str) {
        self.messages.push(String::from(message));
    }

    pub fn get_messages(&self) -> &[String] {
        &self.messages
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }
}
