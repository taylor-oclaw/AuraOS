extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct OfflineConflictResolve {
    conflicts: Vec<String>,
}

impl OfflineConflictResolve {
    pub fn new() -> Self {
        OfflineConflictResolve {
            conflicts: Vec::new(),
        }
    }

    pub fn add_conflict(&mut self, conflict: String) {
        self.conflicts.push(conflict);
    }

    pub fn remove_conflict(&mut self, index: usize) -> Option<String> {
        if index < self.conflicts.len() {
            Some(self.conflicts.remove(index))
        } else {
            None
        }
    }

    pub fn get_conflict(&self, index: usize) -> Option<&String> {
        self.conflicts.get(index)
    }

    pub fn list_conflicts(&self) -> &Vec<String> {
        &self.conflicts
    }

    pub fn resolve_all_conflicts(&mut self) {
        self.conflicts.clear();
    }
}
