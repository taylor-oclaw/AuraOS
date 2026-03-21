extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailReplyToneMatcher {
    tone_database: Vec<(String, String)>,
}

impl EmailReplyToneMatcher {
    pub fn new() -> Self {
        EmailReplyToneMatcher {
            tone_database: Vec::new(),
        }
    }

    pub fn add_tone(&mut self, input_tone: &str, reply_tone: &str) {
        let input_tone = String::from(input_tone);
        let reply_tone = String::from(reply_tone);
        self.tone_database.push((input_tone, reply_tone));
    }

    pub fn get_reply_tone(&self, input_text: &str) -> Option<&String> {
        for (input_tone, reply_tone) in &self.tone_database {
            if input_text.contains(input_tone) {
                return Some(reply_tone);
            }
        }
        None
    }

    pub fn remove_tone(&mut self, input_tone: &str) -> bool {
        let pos = self.tone_database.iter().position(|(key, _)| key == input_tone);
        if let Some(position) = pos {
            self.tone_database.remove(position);
            true
        } else {
            false
        }
    }

    pub fn list_all_tones(&self) -> Vec<&String> {
        self.tone_database.iter().map(|(_, reply)| reply).collect()
    }

    pub fn clear_database(&mut self) {
        self.tone_database.clear();
    }
}
