extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

mod ai_agent_protocol_v2 {
    use super::*;

    pub struct AIProtocolV2 {
        agent_id: String,
        capabilities: Vec<String>,
        status: String,
        messages: Vec<String>,
        version: u32,
    }

    impl AIProtocolV2 {
        pub fn new(agent_id: &str) -> Self {
            AIProtocolV2 {
                agent_id: String::from(agent_id),
                capabilities: Vec::new(),
                status: String::from("idle"),
                messages: Vec::new(),
                version: 2,
            }
        }

        pub fn add_capability(&mut self, capability: &str) {
            self.capabilities.push(String::from(capability));
        }

        pub fn get_capabilities(&self) -> &[String] {
            &self.capabilities
        }

        pub fn update_status(&mut self, status: &str) {
            self.status = String::from(status);
        }

        pub fn get_status(&self) -> &str {
            &self.status
        }

        pub fn add_message(&mut self, message: &str) {
            self.messages.push(String::from(message));
        }

        pub fn get_messages(&self) -> &[String] {
            &self.messages
        }

        pub fn get_version(&self) -> u32 {
            self.version
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_protocol_v2() {
        let mut protocol = ai_agent_protocol_v2::AIProtocolV2::new("agent1");

        assert_eq!(protocol.get_version(), 2);
        assert_eq!(protocol.get_status(), "idle");
        assert!(protocol.get_capabilities().is_empty());
        assert!(protocol.get_messages().is_empty());

        protocol.add_capability("language_processing");
        protocol.add_capability("image_recognition");
        assert_eq!(protocol.get_capabilities().len(), 2);

        protocol.update_status("active");
        assert_eq!(protocol.get_status(), "active");

        protocol.add_message("Hello, world!");
        assert_eq!(protocol.get_messages().len(), 1);
    }
}
