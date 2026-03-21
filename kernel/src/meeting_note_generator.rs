extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingNoteGenerator {
    notes: Vec<String>,
}

impl MeetingNoteGenerator {
    pub fn new() -> Self {
        MeetingNoteGenerator { notes: Vec::new() }
    }

    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    pub fn get_notes(&self) -> &Vec<String> {
        &self.notes
    }

    pub fn clear_notes(&mut self) {
        self.notes.clear();
    }

    pub fn find_note(&self, keyword: &str) -> Option<&String> {
        self.notes.iter().find(|note| note.contains(keyword))
    }

    pub fn count_notes(&self) -> usize {
        self.notes.len()
    }
}
