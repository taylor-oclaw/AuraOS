extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let resolver = FamilyHubConflictResolver::new();
    resolver.resolve_conflict("kitchen");
}

pub struct FamilyHubConflictResolver {
    conflicts: Vec<String>,
}

impl FamilyHubConflictResolver {
    pub fn new() -> Self {
        FamilyHubConflictResolver {
            conflicts: Vec::new(),
        }
    }

    pub fn add_conflict(&mut self, conflict: &str) {
        self.conflicts.push(String::from(conflict));
    }

    pub fn remove_conflict(&mut self, conflict: &str) {
        if let Some(index) = self.conflicts.iter().position(|c| c == conflict) {
            self.conflicts.remove(index);
        }
    }

    pub fn list_conflicts(&self) -> Vec<String> {
        self.conflicts.clone()
    }

    pub fn resolve_conflict(&self, location: &str) {
        match location {
        }
    }

    pub fn has_conflict(&self, conflict: &str) -> bool {
        self.conflicts.contains(&String::from(conflict))
    }
}
