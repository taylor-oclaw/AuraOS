extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_tts_voice_bank_init() {
    // Initialization logic for the voice bank module
}

#[no_mangle]
pub extern "C" fn lang_tts_voice_bank_exit() {
    // Cleanup logic for the voice bank module
}

pub struct VoiceBank {
    voices: Vec<String>,
}

impl VoiceBank {
    pub fn new() -> Self {
        VoiceBank {
            voices: Vec::new(),
        }
    }

    pub fn add_voice(&mut self, name: String) {
        self.voices.push(name);
    }

    pub fn remove_voice(&mut self, index: usize) -> Option<String> {
        if index < self.voices.len() {
            Some(self.voices.remove(index))
        } else {
            None
        }
    }

    pub fn get_voice(&self, index: usize) -> Option<&String> {
        self.voices.get(index)
    }

    pub fn list_voices(&self) -> &[String] {
        &self.voices
    }

    pub fn count_voices(&self) -> usize {
        self.voices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_bank() {
        let mut bank = VoiceBank::new();
        assert_eq!(bank.count_voices(), 0);

        bank.add_voice(String::from("Alice"));
        bank.add_voice(String::from("Bob"));
        assert_eq!(bank.count_voices(), 2);
        assert_eq!(bank.get_voice(0), Some(&String::from("Alice")));
        assert_eq!(bank.get_voice(1), Some(&String::from("Bob")));

        let removed = bank.remove_voice(0);
        assert_eq!(removed, Some(String::from("Alice")));
        assert_eq!(bank.count_voices(), 1);

        let voices: Vec<&String> = bank.list_voices().iter().collect();
        assert_eq!(voices, vec![&String::from("Bob")]);
    }
}
