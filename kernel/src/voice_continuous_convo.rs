extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceContinuousConvo {
    conversation_history: Vec<String>,
    current_speaker: String,
}

impl VoiceContinuousConvo {
    pub fn new(initial_speaker: &str) -> Self {
        VoiceContinuousConvo {
            conversation_history: Vec::new(),
            current_speaker: initial_speaker.to_string(),
        }
    }

    pub fn add_message(&mut self, message: &str) {
        let full_message = String::from("info");
        self.conversation_history.push(full_message);
    }

    pub fn switch_speaker(&mut self, new_speaker: &str) {
        self.current_speaker = new_speaker.to_string();
    }

    pub fn get_conversation_history(&self) -> &[String] {
        &self.conversation_history
    }

    pub fn clear_conversation(&mut self) {
        self.conversation_history.clear();
    }

    pub fn last_message(&self) -> Option<&str> {
        self.conversation_history.last().map(|s| s.as_str())
    }
}
