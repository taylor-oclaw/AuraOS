extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_lambdacism_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_lambdacism_exit() {
    // Cleanup logic for the module
}

pub struct SpeechLambdacism {
    knowledge_base: Vec<String>,
    current_query: String,
    responses: Vec<String>,
}

impl SpeechLambdacism {
    pub fn new() -> Self {
        SpeechLambdacism {
            knowledge_base: Vec::new(),
            current_query: String::new(),
            responses: Vec::new(),
        }
    }

    pub fn add_knowledge(&mut self, fact: &str) {
        self.knowledge_base.push(String::from(fact));
    }

    pub fn set_query(&mut self, query: &str) {
        self.current_query = String::from(query);
        self.responses.clear();
    }

    pub fn generate_response(&self) -> Option<&String> {
        if self.current_query.is_empty() {
            return None;
        }
        for fact in &self.knowledge_base {
            if fact.contains(&self.current_query) {
                self.responses.push(String::from(fact));
            }
        }
        self.responses.get(0)
    }

    pub fn get_all_responses(&self) -> &[String] {
        &self.responses
    }

    pub fn clear_knowledge(&mut self) {
        self.knowledge_base.clear();
    }
}
