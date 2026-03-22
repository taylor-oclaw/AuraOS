extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[allow(non_camel_case_types)]
pub struct people_source_handwritten_note {
    notes: Vec<String>,
}

impl people_source_handwritten_note {
    pub fn new() -> Self {
        people_source_handwritten_note {
            notes: Vec::new(),
        }
    }

    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    pub fn get_notes(&self) -> &Vec<String> {
        &self.notes
    }

    pub fn remove_note(&mut self, index: usize) -> Option<String> {
        if index < self.notes.len() {
            Some(self.notes.remove(index))
        } else {
            None
        }
    }

    pub fn find_notes_with_keyword(&self, keyword: &str) -> Vec<&String> {
        self.notes.iter().filter(|note| note.contains(keyword)).collect()
    }

    pub fn clear_notes(&mut self) {
        self.notes.clear();
    }
}