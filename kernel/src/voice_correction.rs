extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct VoiceCorrection {
    corrections: Vec<(String, String)>,
}

impl VoiceCorrection {
    pub fn new() -> Self {
        VoiceCorrection {
            corrections: Vec::new(),
        }
    }

    pub fn add_correction(&mut self, from: &str, to: &str) {
        let from_str = String::from(from);
        let to_str = String::from(to);
        self.corrections.push((from_str, to_str));
    }

    pub fn correct_text(&self, text: &str) -> String {
        let mut corrected_text = String::from(text);
        for (from, to) in &self.corrections {
            while corrected_text.contains(from) {
                corrected_text.replace_range(corrected_text.find(from).unwrap()..corrected_text.find(from).unwrap() + from.len(), to);
            }
        }
        corrected_text
    }

    pub fn remove_correction(&mut self, from: &str) -> bool {
        let pos = self.corrections.iter().position(|&(ref f, _)| f == from);
        if let Some(index) = pos {
            self.corrections.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_corrections(&self) -> Vec<(String, String)> {
        self.corrections.clone()
    }

    pub fn clear_corrections(&mut self) {
        self.corrections.clear();
    }
}
