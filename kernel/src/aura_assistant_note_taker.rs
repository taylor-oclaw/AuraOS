extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code if needed
}

pub struct NoteTaker {
    notes: Vec<String>,
}

impl NoteTaker {
    pub fn new() -> Self {
        NoteTaker {
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

    pub fn clear_notes(&mut self) {
        self.notes.clear();
    }

    pub fn note_count(&self) -> usize {
        self.notes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_taker() {
        let mut nt = NoteTaker::new();
        assert_eq!(nt.note_count(), 0);

        nt.add_note(String::from("First note"));
        nt.add_note(String::from("Second note"));
        assert_eq!(nt.note_count(), 2);

        let notes = nt.get_notes();
        assert_eq!(notes[0], "First note");
        assert_eq!(notes[1], "Second note");

        let removed_note = nt.remove_note(0);
        assert_eq!(removed_note, Some(String::from("First note")));
        assert_eq!(nt.note_count(), 1);

        nt.clear_notes();
        assert_eq!(nt.note_count(), 0);
    }
}
