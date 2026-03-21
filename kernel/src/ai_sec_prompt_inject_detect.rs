extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct PromptInjectionDetector {
    known_prompts: Vec<String>,
    detected_injections: Vec<String>,
}

impl PromptInjectionDetector {
    pub fn new() -> Self {
        PromptInjectionDetector {
            known_prompts: Vec::new(),
            detected_injections: Vec::new(),
        }
    }

    pub fn add_known_prompt(&mut self, prompt: String) {
        self.known_prompts.push(prompt);
    }

    pub fn detect_injection(&mut self, input: &str) -> bool {
        for prompt in &self.known_prompts {
            if input.contains(prompt) {
                self.detected_injections.push(input.to_string());
                return true;
            }
        }
        false
    }

    pub fn get_detected_injections(&self) -> &[String] {
        &self.detected_injections
    }

    pub fn clear_detected_injections(&mut self) {
        self.detected_injections.clear();
    }

    pub fn count_known_prompts(&self) -> usize {
        self.known_prompts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_detection() {
        let mut detector = PromptInjectionDetector::new();
        detector.add_known_prompt(String::from("inject"));
        assert_eq!(detector.detect_injection("This is a test inject"), true);
        assert_eq!(detector.get_detected_injections().len(), 1);
        detector.clear_detected_injections();
        assert_eq!(detector.get_detected_injections().len(), 0);
    }
}
