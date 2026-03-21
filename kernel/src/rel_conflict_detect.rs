extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut detector = RelConflictDetector::new();
    detector.add_relationship("A", "B");
    detector.add_relationship("B", "C");
    detector.add_relationship("C", "D");
    detector.add_relationship("D", "E");

    println!("Is A related to C? {}", detector.are_related("A", "C")); // Should print true
    println!("Is B related to D? {}", detector.are_related("B", "D")); // Should print true
    println!("Is E related to A? {}", detector.are_related("E", "A")); // Should print false

    detector.remove_relationship("B", "C");
    println!("Is B still related to C? {}", detector.are_related("B", "C")); // Should print false
}

pub struct RelConflictDetector {
    relationships: Vec<(String, String)>,
}

impl RelConflictDetector {
    pub fn new() -> Self {
        RelConflictDetector {
            relationships: Vec::new(),
        }
    }

    pub fn add_relationship(&mut self, from: &str, to: &str) {
        self.relationships.push((from.to_string(), to.to_string()));
    }

    pub fn remove_relationship(&mut self, from: &str, to: &str) {
        self.relationships.retain(|&(ref f, ref t)| f != from || t != to);
    }

    pub fn are_related(&self, from: &str, to: &str) -> bool {
        if let Some(index) = self.find_relationship(from, to) {
            return true;
        }
        false
    }

    fn find_relationship(&self, from: &str, to: &str) -> Option<usize> {
        for (i, &(ref f, ref t)) in self.relationships.iter().enumerate() {
            if f == from && t == to {
                return Some(i);
            }
        }
        None
    }

    pub fn get_all_relationships(&self) -> Vec<(String, String)> {
        self.relationships.clone()
    }
}
