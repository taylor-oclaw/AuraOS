extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct PetTracker {
    pets: Vec<String>,
}

impl PetTracker {
    pub fn new() -> Self {
        PetTracker { pets: Vec::new() }
    }

    pub fn add_pet(&mut self, name: String) {
        self.pets.push(name);
    }

    pub fn remove_pet(&mut self, name: &str) -> bool {
        if let Some(index) = self.pets.iter().position(|pet| pet == name) {
            self.pets.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_pets(&self) -> Vec<String> {
        self.pets.clone()
    }

    pub fn find_pet(&self, name: &str) -> Option<&String> {
        self.pets.iter().find(|pet| pet == &name)
    }

    pub fn count_pets(&self) -> usize {
        self.pets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_tracker() {
        let mut tracker = PetTracker::new();
        assert_eq!(tracker.count_pets(), 0);

        tracker.add_pet(String::from("Buddy"));
        tracker.add_pet(String::from("Charlie"));
        assert_eq!(tracker.count_pets(), 2);

        assert!(tracker.remove_pet("Buddy"));
        assert_eq!(tracker.count_pets(), 1);
        assert!(!tracker.remove_pet("Buddy"));

        let pets = tracker.list_pets();
        assert_eq!(pets, vec![String::from("Charlie")]);

        assert_eq!(tracker.find_pet("Charlie"), Some(&String::from("Charlie")));
        assert_eq!(tracker.find_pet("Buddy"), None);
    }
}
