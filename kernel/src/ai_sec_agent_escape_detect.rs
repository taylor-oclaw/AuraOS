extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("AI Security Agent Escape Detection Module Loaded");
    0
}

pub struct AISEscapeDetector {
    escape_sequences: Vec<String>,
    detected_escapes: usize,
}

impl AISEscapeDetector {
    pub fn new() -> Self {
        AISEscapeDetector {
            escape_sequences: vec![
                String::from("escape1"),
                String::from("escape2"),
                String::from("escape3"),
                String::from("escape4"),
                String::from("escape5"),
            ],
            detected_escapes: 0,
        }
    }

    pub fn add_escape_sequence(&mut self, sequence: &str) {
        self.escape_sequences.push(String::from(sequence));
    }

    pub fn remove_escape_sequence(&mut self, sequence: &str) -> bool {
        if let Some(index) = self.escape_sequences.iter().position(|s| s == sequence) {
            self.escape_sequences.remove(index);
            true
        } else {
            false
        }
    }

    pub fn detect_escape(&mut self, input: &str) -> bool {
        for sequence in &self.escape_sequences {
            if input.contains(sequence) {
                self.detected_escapes += 1;
                return true;
            }
        }
        false
    }

    pub fn get_detected_escapes_count(&self) -> usize {
        self.detected_escapes
    }

    pub fn reset_detected_escapes(&mut self) {
        self.detected_escapes = 0;
    }
}
