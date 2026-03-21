extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_standard_clauses_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_standard_clauses_exit() {
    // Cleanup logic for the module
}

pub struct MeshStandardClauses {
    clauses: Vec<String>,
}

impl MeshStandardClauses {
    pub fn new() -> Self {
        MeshStandardClauses {
            clauses: Vec::new(),
        }
    }

    pub fn add_clause(&mut self, clause: String) {
        self.clauses.push(clause);
    }

    pub fn remove_clause(&mut self, index: usize) -> Option<String> {
        if index < self.clauses.len() {
            Some(self.clauses.remove(index))
        } else {
            None
        }
    }

    pub fn get_clause(&self, index: usize) -> Option<&String> {
        self.clauses.get(index)
    }

    pub fn list_clauses(&self) -> &[String] {
        &self.clauses
    }

    pub fn clear_clauses(&mut self) {
        self.clauses.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_standard_clauses() {
        let mut clauses = MeshStandardClauses::new();

        assert_eq!(clauses.list_clauses().len(), 0);

        clauses.add_clause(String::from("Clause 1"));
        clauses.add_clause(String::from("Clause 2"));

        assert_eq!(clauses.list_clauses().len(), 2);
        assert_eq!(clauses.get_clause(0), Some(&String::from("Clause 1")));
        assert_eq!(clauses.get_clause(1), Some(&String::from("Clause 2")));

        let removed = clauses.remove_clause(0);
        assert_eq!(removed, Some(String::from("Clause 1")));
        assert_eq!(clauses.list_clauses().len(), 1);

        clauses.clear_clauses();
        assert_eq!(clauses.list_clauses().len(), 0);
    }
}
