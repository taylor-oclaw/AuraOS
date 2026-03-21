extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceResponseSelect {
    responses: Vec<String>,
}

impl VoiceResponseSelect {
    pub fn new() -> Self {
        VoiceResponseSelect {
            responses: Vec::new(),
        }
    }

    pub fn add_response(&mut self, response: String) {
        self.responses.push(response);
    }

    pub fn remove_response(&mut self, index: usize) -> Option<String> {
        if index < self.responses.len() {
            Some(self.responses.remove(index))
        } else {
            None
        }
    }

    pub fn get_response(&self, index: usize) -> Option<&String> {
        self.responses.get(index)
    }

    pub fn list_responses(&self) -> &[String] {
        &self.responses
    }

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
}
