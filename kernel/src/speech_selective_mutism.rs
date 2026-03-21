extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechSelectiveMutism {
    user_id: u32,
    is_active: bool,
    allowed_phrases: Vec<String>,
    blocked_phrases: Vec<String>,
    log: Vec<String>,
}

impl SpeechSelectiveMutism {
    pub fn new(user_id: u32) -> Self {
        SpeechSelectiveMutism {
            user_id,
            is_active: false,
            allowed_phrases: Vec::new(),
            blocked_phrases: Vec::new(),
            log: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.log.push(String::from("info"));
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.log.push(String::from("info"));
    }

    pub fn add_allowed_phrase(&mut self, phrase: String) {
        self.allowed_phrases.push(phrase);
        self.log.push(String::from("info").unwrap());
    }

    pub fn add_blocked_phrase(&mut self, phrase: String) {
        self.blocked_phrases.push(phrase);
        self.log.push(String::from("info").unwrap());
    }

    pub fn can_speak(&self, phrase: &str) -> bool {
        if !self.is_active {
            return true;
        }
        if self.allowed_phrases.contains(&phrase.to_string()) {
            return true;
        }
        if self.blocked_phrases.contains(&phrase.to_string()) {
            return false;
        }
        true
    }

    pub fn get_log(&self) -> &Vec<String> {
        &self.log
    }
}
