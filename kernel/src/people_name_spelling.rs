extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn people_name_spelling_init() {
    // Initialization code if needed
}

pub extern "C" fn people_name_spelling_exit() {
    // Cleanup code if needed
}

pub struct PeopleNameSpelling {
    names: Vec<String>,
}

impl PeopleNameSpelling {
    pub fn new() -> Self {
        PeopleNameSpelling {
            names: Vec::new(),
        }
    }

    pub fn add_name(&mut self, name: &str) {
        self.names.push(String::from(name));
    }

    pub fn remove_name(&mut self, name: &str) {
        if let Some(index) = self.names.iter().position(|n| n == name) {
            self.names.remove(index);
        }
    }

    pub fn get_names(&self) -> Vec<String> {
        self.names.clone()
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.names.contains(&String::from(name))
    }

    pub fn count_names(&self) -> usize {
        self.names.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_name_spelling() {
        let mut spelling = PeopleNameSpelling::new();
        assert_eq!(spelling.count_names(), 0);

        spelling.add_name("Alice");
        spelling.add_name("Bob");
        assert_eq!(spelling.count_names(), 2);
        assert!(spelling.contains_name("Alice"));
        assert!(!spelling.contains_name("Charlie"));

        let names = spelling.get_names();
        assert_eq!(names, vec![String::from("Alice"), String::from("Bob")]);

        spelling.remove_name("Alice");
        assert_eq!(spelling.count_names(), 1);
        assert!(!spelling.contains_name("Alice"));
    }
}
