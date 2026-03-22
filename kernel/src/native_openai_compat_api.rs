extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NativeOpenaiCompatApi {
    api_key: String,
}

impl NativeOpenaiCompatApi {
    pub fn new(api_key: &str) -> Self {
        NativeOpenaiCompatApi {
            api_key: String::from(api_key),
        }
    }

    pub fn get_api_key(&self) -> &str {
        self.api_key.as_str()
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = String::from(api_key);
    }

    pub fn create_completion(&self, prompt: &str) -> String {
        // Simulate a completion request to the OpenAI API
        format!("Completion for '{}'", prompt)
    }

    pub fn get_model_list(&self) -> Vec<String> {
        vec!["text-davinci-003".to_string(), "text-curie-001".to_string()]
    }

    pub fn create_chat_completion(&self, prompt: &str, model_id: &str) -> String {
        // Simulate a chat completion request to the OpenAI API
        format!("Chat Completion for '{}', using model {}", prompt, model_id)
    }
}