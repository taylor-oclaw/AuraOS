extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn tone_draft_tone_check_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn tone_draft_tone_check_exit() {
    // Cleanup code for the module
}

pub struct ToneCheck {
    tones: Vec<String>,
}

impl ToneCheck {
    pub fn new() -> Self {
        ToneCheck {
            tones: Vec::new(),
        }
    }

    pub fn add_tone(&mut self, tone: String) {
        self.tones.push(tone);
    }

    pub fn remove_tone(&mut self, index: usize) -> Option<String> {
        if index < self.tones.len() {
            Some(self.tones.remove(index))
        } else {
            None
        }
    }

    pub fn get_tone(&self, index: usize) -> Option<&String> {
        self.tones.get(index)
    }

    pub fn list_tones(&self) -> &[String] {
        &self.tones
    }

    pub fn clear_tones(&mut self) {
        self.tones.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_check() {
        let mut tone_check = ToneCheck::new();

        assert_eq!(tone_check.list_tones().len(), 0);

        tone_check.add_tone(String::from("Happy"));
        tone_check.add_tone(String::from("Sad"));

        assert_eq!(tone_check.list_tones().len(), 2);
        assert_eq!(tone_check.get_tone(0), Some(&String::from("Happy")));
        assert_eq!(tone_check.get_tone(1), Some(&String::from("Sad")));

        let removed = tone_check.remove_tone(0);
        assert_eq!(removed, Some(String::from("Happy")));
        assert_eq!(tone_check.list_tones().len(), 1);

        tone_check.clear_tones();
        assert_eq!(tone_check.list_tones().len(), 0);
    }
}
