extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_phoneme_map_custom_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_phoneme_map_custom_exit() {
    // Cleanup logic for the module
}

pub struct SpeechPhonemeMapCustom {
    phoneme_to_id: Vec<(String, u32)>,
    id_to_phoneme: Vec<String>,
}

impl SpeechPhonemeMapCustom {
    pub fn new() -> Self {
        SpeechPhonemeMapCustom {
            phoneme_to_id: Vec::new(),
            id_to_phoneme: Vec::new(),
        }
    }

    pub fn add_phoneme(&mut self, phoneme: &str) -> u32 {
        let id = self.id_to_phoneme.len() as u32;
        self.phoneme_to_id.push((String::from(phoneme), id));
        self.id_to_phoneme.push(String::from(phoneme));
        id
    }

    pub fn get_id_by_phoneme(&self, phoneme: &str) -> Option<u32> {
        for (p, id) in &self.phoneme_to_id {
            if p == phoneme {
                return Some(*id);
            }
        }
        None
    }

    pub fn get_phoneme_by_id(&self, id: u32) -> Option<&str> {
        if id < self.id_to_phoneme.len() as u32 {
            Some(&self.id_to_phoneme[id as usize])
        } else {
            None
        }
    }

    pub fn remove_phoneme(&mut self, phoneme: &str) -> bool {
        let pos = self.phoneme_to_id.iter().position(|(p, _)| p == phoneme);
        if let Some(pos) = pos {
            let id = self.phoneme_to_id[pos].1;
            self.phoneme_to_id.remove(pos);
            self.id_to_phoneme[id as usize] = String::new(); // Mark as removed
            true
        } else {
            false
        }
    }

    pub fn list_all_phonemes(&self) -> Vec<&str> {
        self.id_to_phoneme.iter().filter(|&&p| !p.is_empty()).map(|&p| p.as_str()).collect()
    }
}
