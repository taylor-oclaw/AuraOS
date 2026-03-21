extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut favor_reciprocator = FavorReciprocate::new();
    favor_reciprocator.add_favor("Alice", "Bob");
    favor_reciprocator.add_favor("Bob", "Charlie");
    favor_reciprocator.add_favor("Charlie", "David");

    if favor_reciprocator.has_favor("Alice", "Bob") {
    }

    let reciprocated = favor_reciprocator.reciprocate_favors();
    for (from, to) in reciprocated.iter() {
    }
}

pub struct FavorReciprocate {
    favors: Vec<(String, String)>,
}

impl FavorReciprocate {
    pub fn new() -> Self {
        FavorReciprocate {
            favors: Vec::new(),
        }
    }

    pub fn add_favor(&mut self, from: &str, to: &str) {
        self.favors.push((String::from(from), String::from(to)));
    }

    pub fn has_favor(&self, from: &str, to: &str) -> bool {
        self.favors.iter().any(|&(ref f, ref t)| f == from && t == to)
    }

    pub fn remove_favor(&mut self, from: &str, to: &str) {
        self.favors.retain(|&(ref f, ref t)| !(f == from && t == to));
    }

    pub fn list_favors(&self) -> Vec<(String, String)> {
        self.favors.clone()
    }

    pub fn reciprocate_favors(&mut self) -> Vec<(String, String)> {
        let mut reciprocated = Vec::new();
        for (from, to) in self.favors.iter().cloned() {
            if !self.has_favor(to, from) {
                reciprocated.push((to.clone(), from.clone()));
                self.add_favor(to, from);
            }
        }
        reciprocated
    }
}
