extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentPromptOptimizer {
    prompts: Vec<String>,
}

impl AgentPromptOptimizer {
    pub fn new() -> Self {
        AgentPromptOptimizer {
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

    pub fn optimize_prompts(&mut self) {
        // Simple optimization: remove duplicates
        let mut unique_prompts = Vec::new();
        for prompt in &self.prompts {
            if !unique_prompts.contains(prompt) {
                unique_prompts.push(prompt.clone());
            }
        }
        self.prompts = unique_prompts;
    }

    pub fn list_prompts(&self) -> &[String] {
        &self.prompts
    }
}
