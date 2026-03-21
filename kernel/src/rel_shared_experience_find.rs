extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_shared_experience_find_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_shared_experience_find_exit() {
    // Cleanup logic for the module
}

pub struct SharedExperience {
    experiences: Vec<String>,
}

impl SharedExperience {
    pub fn new() -> Self {
        SharedExperience {
            experiences: Vec::new(),
        }
    }

    pub fn add_experience(&mut self, experience: String) {
        self.experiences.push(experience);
    }

    pub fn get_experiences(&self) -> &Vec<String> {
        &self.experiences
    }

    pub fn find_experience(&self, keyword: &str) -> Vec<&String> {
        self.experiences.iter().filter(|exp| exp.contains(keyword)).collect()
    }

    pub fn remove_experience(&mut self, index: usize) -> Option<String> {
        if index < self.experiences.len() {
            Some(self.experiences.remove(index))
        } else {
            None
        }
    }

    pub fn count_experiences(&self) -> usize {
        self.experiences.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_experience() {
        let mut se = SharedExperience::new();
        se.add_experience(String::from("Learned Rust programming"));
        se.add_experience(String::from("Explored AI concepts"));

        assert_eq!(se.count_experiences(), 2);
        assert_eq!(se.get_experiences().len(), 2);

        let found = se.find_experience("Rust");
        assert_eq!(found.len(), 1);
        assert_eq!(*found[0], "Learned Rust programming");

        let removed = se.remove_experience(0);
        assert_eq!(removed, Some(String::from("Learned Rust programming")));
        assert_eq!(se.count_experiences(), 1);

        let found_empty = se.find_experience("Rust");
        assert_eq!(found_empty.len(), 0);
    }
}
