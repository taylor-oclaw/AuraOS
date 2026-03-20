extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

struct AuraNotes {
    notes: Vec<String>,
}

impl AuraNotes {
    pub fn new() -> Self {
        AuraNotes { notes: Vec::new() }
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

    pub fn find_note(&self, query: &str) -> Vec<&String> {
        self.notes.iter().filter(|note| note.contains(query)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_notes() {
        let mut aura_notes = AuraNotes::new();
        aura_notes.add_note(String::from("Meeting at 10 AM"));
        aura_notes.add_note(String::from("Buy groceries"));

        assert_eq!(aura_notes.get_notes().len(), 2);
        assert_eq!(aura_notes.remove_note(0), Some(String::from("Meeting at 10 AM")));
        assert_eq!(aura_notes.get_notes().len(), 1);

        aura_notes.clear_notes();
        assert_eq!(aura_notes.get_notes().len(), 0);

        aura_notes.add_note(String::from("AI research"));
        aura_notes.add_note(String::from("Machine learning"));
        let found_notes = aura_notes.find_note("AI");
        assert_eq!(found_notes.len(), 1);
    }
}
