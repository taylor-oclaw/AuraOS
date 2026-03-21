extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut wake_word = VoiceWakeWord::new(String::from("Alexa"));
    wake_word.train(vec!["echo", "repeat", "play"]);
    wake_word.add_keyword("music");
    wake_word.remove_keyword("repeat");
}

pub struct VoiceWakeWord {
    name: String,
    keywords: Vec<String>,
}

impl VoiceWakeWord {
    pub fn new(name: String) -> Self {
        VoiceWakeWord {
            name,
            keywords: Vec::new(),
        }
    }

    pub fn train(&mut self, commands: Vec<&str>) {
        for command in commands {
            if !self.keywords.contains(&command.to_string()) {
                self.keywords.push(command.to_string());
            }
        }
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        if !self.keywords.contains(&keyword.to_string()) {
            self.keywords.push(keyword.to_string());
        }
    }

    pub fn remove_keyword(&mut self, keyword: &str) {
        self.keywords.retain(|k| k != keyword);
    }

    pub fn is_keyword(&self, keyword: &str) -> bool {
        self.keywords.contains(&keyword.to_string())
    }

    pub fn get_keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }
}
