extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct SpeechSituationalAdapt {
    context: String,
    responses: Vec<String>,
}

impl SpeechSituationalAdapt {
    pub fn new(context: &str) -> Self {
        SpeechSituationalAdapt {
            context: String::from(context),
            responses: Vec::new(),
        }
    }

    pub fn add_response(&mut self, response: &str) {
        self.responses.push(String::from(response));
    }

    pub fn get_context(&self) -> &String {
        &self.context
    }

    pub fn set_context(&mut self, context: &str) {
        self.context = String::from(context);
    }

    pub fn get_responses(&self) -> &Vec<String> {
        &self.responses
    }

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
}
