extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_relationship_neglect_warn_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_relationship_neglect_warn_exit() {
    // Cleanup logic for the module
}

pub struct RelationshipNeglectWarning {
    warnings: Vec<String>,
}

impl RelationshipNeglectWarning {
    pub fn new() -> Self {
        RelationshipNeglectWarning {
            warnings: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn get_warnings(&self) -> &Vec<String> {
        &self.warnings
    }

    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_neglect_warning() {
        let mut rnw = RelationshipNeglectWarning::new();
        assert_eq!(rnw.warning_count(), 0);
        assert!(!rnw.has_warnings());

        rnw.add_warning(String::from("Neglected relationship with Alice"));
        rnw.add_warning(String::from("Neglected relationship with Bob"));

        assert_eq!(rnw.warning_count(), 2);
        assert!(rnw.has_warnings());

        let warnings = rnw.get_warnings();
        assert_eq!(warnings.len(), 2);
        assert_eq!(warnings[0], "Neglected relationship with Alice");
        assert_eq!(warnings[1], "Neglected relationship with Bob");

        rnw.clear_warnings();
        assert_eq!(rnw.warning_count(), 0);
        assert!(!rnw.has_warnings());
    }
}
