extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAssistantContext {
    user_id: String,
    session_history: Vec<String>,
    current_query: String,
    response_buffer: String,
    system_status: String,
}

impl AuraAssistantContext {
    pub fn new(user_id: &str) -> Self {
        AuraAssistantContext {
            user_id: String::from(user_id),
            session_history: Vec::new(),
            current_query: String::new(),
            response_buffer: String::new(),
            system_status: String::from("Active"),
        }
    }

    pub fn set_current_query(&mut self, query: &str) {
        self.current_query = String::from(query);
    }

    pub fn get_current_query(&self) -> &str {
        &self.current_query
    }

    pub fn add_to_session_history(&mut self, entry: &str) {
        self.session_history.push(String::from(entry));
    }

    pub fn clear_response_buffer(&mut self) {
        self.response_buffer.clear();
    }

    pub fn append_to_response_buffer(&mut self, text: &str) {
        self.response_buffer.push_str(text);
    }

    pub fn get_response_buffer(&self) -> &str {
        &self.response_buffer
    }

    pub fn set_system_status(&mut self, status: &str) {
        self.system_status = String::from(status);
    }

    pub fn get_system_status(&self) -> &str {
        &self.system_status
    }
}
