extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneContextPersonal {
    user_id: String,
    tone_history: Vec<String>,
}

impl ToneContextPersonal {
    pub fn new(user_id: &str) -> Self {
        ToneContextPersonal {
            user_id: String::from(user_id),
            tone_history: Vec::new(),
        }
    }

    pub fn add_tone(&mut self, tone: &str) {
        self.tone_history.push(String::from(tone));
    }

    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }

    pub fn get_last_tone(&self) -> Option<&String> {
        self.tone_history.last()
    }

    pub fn clear_tones(&mut self) {
        self.tone_history.clear();
    }

    pub fn list_tones(&self) -> &[String] {
        &self.tone_history
    }
}
