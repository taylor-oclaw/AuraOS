extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MCPromptHandler {
    prompts: Vec<String>,
}

impl MCPromptHandler {
    pub fn new() -> Self {
        MCPromptHandler {
            prompts: Vec::new(),
        }
    }

    pub fn add_prompt(&mut self, prompt: String) {
        self.prompts.push(prompt);
    }

    pub fn remove_prompt(&mut self, index: usize) -> Option<String> {
        if index < self.prompts.len() {
            Some(self.prompts.remove(index))
        } else {
            None
        }
    }

    pub fn get_prompt(&self, index: usize) -> Option<&String> {
        self.prompts.get(index)
    }

    pub fn list_prompts(&self) -> &[String] {
        &self.prompts
    }

    pub fn clear_prompts(&mut self) {
        self.prompts.clear();
    }
}
