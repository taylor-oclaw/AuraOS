extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_accent_korean_init() {
    // Initialization logic for the kernel module
}

pub extern "C" fn lang_accent_korean_exit() {
    // Cleanup logic for the kernel module
}

pub struct KoreanAccent {
    dictionary: Vec<(String, String)>,
}

impl KoreanAccent {
    pub fn new() -> Self {
        KoreanAccent {
            dictionary: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, original: &str, accented: &str) {
        self.dictionary.push((String::from(original), String::from(accented)));
    }

    pub fn get_accented(&self, word: &str) -> Option<&String> {
        for (original, accented) in &self.dictionary {
            if original == word {
                return Some(accented);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, word: &str) -> bool {
        let pos = self.dictionary.iter().position(|(original, _)| original == word);
        if let Some(index) = pos {
            self.dictionary.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_entries(&self) -> Vec<&String> {
        self.dictionary.iter().map(|(_, accented)| accented).collect()
    }

    pub fn clear_dictionary(&mut self) {
        self.dictionary.clear();
    }
}
