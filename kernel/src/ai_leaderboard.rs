extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AIEntry {
    name: String,
    score: u32,
}

impl AIEntry {
    pub fn new(name: &str, score: u32) -> Self {
        AIEntry {
            name: String::from(name),
            score,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }
}

pub struct AILeaderboard {
    entries: Vec<AIEntry>,
}

impl AILeaderboard {
    pub fn new() -> Self {
        AILeaderboard {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, name: &str, score: u32) {
        let entry = AIEntry::new(name, score);
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &[AIEntry] {
        &self.entries
    }

    pub fn find_by_name(&self, name: &str) -> Option<&AIEntry> {
        self.entries.iter().find(|entry| entry.get_name() == name)
    }

    pub fn update_score(&mut self, name: &str, new_score: u32) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.get_name() == name) {
            entry.set_score(new_score);
            true
        } else {
            false
        }
    }

    pub fn remove_entry(&mut self, name: &str) -> bool {
        let pos = self.entries.iter().position(|entry| entry.get_name() == name);
        if let Some(index) = pos {
            self.entries.remove(index);
            true
        } else {
            false
        }
    }
}
