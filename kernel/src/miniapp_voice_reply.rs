extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct MiniAppVoiceReply {
    responses: Vec<String>,
    current_index: usize,
}

impl MiniAppVoiceReply {
    pub fn new(responses: Vec<&str>) -> Self {
        MiniAppVoiceReply {
            responses: responses.into_iter().map(|s| s.to_string()).collect(),
            current_index: 0,
        }
    }

    pub fn add_response(&mut self, response: &str) {
        self.responses.push(response.to_string());
    }

    pub fn remove_response(&mut self, index: usize) -> Option<String> {
        if index < self.responses.len() {
            Some(self.responses.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_response(&self) -> &str {
        if self.responses.is_empty() {
            "No responses available"
        } else {
            &self.responses[self.current_index]
        }
    }

    pub fn next_response(&mut self) -> &str {
        if self.responses.is_empty() {
            "No responses available"
        } else {
            self.current_index = (self.current_index + 1) % self.responses.len();
            &self.responses[self.current_index]
        }
    }

    pub fn previous_response(&mut self) -> &str {
        if self.responses.is_empty() {
            "No responses available"
        } else {
            if self.current_index == 0 {
                self.current_index = self.responses.len() - 1;
            } else {
                self.current_index -= 1;
            }
            &self.responses[self.current_index]
        }
    }
}
