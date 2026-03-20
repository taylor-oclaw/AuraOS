extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentHumanFeedback {
    feedbacks: Vec<String>,
    user_id: u32,
}

impl AgentHumanFeedback {
    pub fn new(user_id: u32) -> Self {
        AgentHumanFeedback {
            feedbacks: Vec::new(),
            user_id,
        }
    }

    pub fn add_feedback(&mut self, feedback: String) {
        self.feedbacks.push(feedback);
    }

    pub fn get_feedbacks(&self) -> &Vec<String> {
        &self.feedbacks
    }

    pub fn clear_feedbacks(&mut self) {
        self.feedbacks.clear();
    }

    pub fn has_feedback(&self, feedback: &str) -> bool {
        self.feedbacks.iter().any(|f| f == feedback)
    }

    pub fn user_id(&self) -> u32 {
        self.user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_human_feedback() {
        let mut agent = AgentHumanFeedback::new(1);
        assert_eq!(agent.user_id(), 1);
        assert!(agent.get_feedbacks().is_empty());

        agent.add_feedback(String::from("Good"));
        assert!(!agent.get_feedbacks().is_empty());
        assert_eq!(agent.get_feedbacks().len(), 1);
        assert!(agent.has_feedback("Good"));

        agent.clear_feedbacks();
        assert!(agent.get_feedbacks().is_empty());
    }
}
