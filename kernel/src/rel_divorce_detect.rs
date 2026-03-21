extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_divorce_detect_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_divorce_detect_exit() {
    // Cleanup logic for the module
}

pub struct RelationshipDivorceDetector {
    relationships: Vec<(String, String)>,
    divorces: Vec<(String, String)>,
}

impl RelationshipDivorceDetector {
    pub fn new() -> Self {
        RelationshipDivorceDetector {
            relationships: Vec::new(),
            divorces: Vec::new(),
        }
    }

    pub fn add_relationship(&mut self, person1: &str, person2: &str) {
        self.relationships.push((String::from(person1), String::from(person2)));
    }

    pub fn record_divorce(&mut self, person1: &str, person2: &str) {
        if let Some(index) = self
            .relationships
            .iter()
            .position(|&(ref p1, ref p2)| p1 == person1 && p2 == person2)
        {
            self.divorces.push(self.relationships.remove(index));
        }
    }

    pub fn is_divorced(&self, person1: &str, person2: &str) -> bool {
        self.divorces.iter().any(|&(ref p1, ref p2)| {
            (p1 == person1 && p2 == person2) || (p1 == person2 && p2 == person1)
        })
    }

    pub fn get_all_relationships(&self) -> Vec<(String, String)> {
        self.relationships.clone()
    }

    pub fn get_all_divorces(&self) -> Vec<(String, String)> {
        self.divorces.clone()
    }
}
