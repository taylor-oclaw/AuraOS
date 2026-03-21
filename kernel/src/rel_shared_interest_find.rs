extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_shared_interest_find_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_shared_interest_find_exit() {
    // Cleanup logic for the module
}

pub struct SharedInterestFinder {
    interests: Vec<String>,
}

impl SharedInterestFinder {
    pub fn new() -> Self {
        SharedInterestFinder {
            interests: Vec::new(),
        }
    }

    pub fn add_interest(&mut self, interest: String) {
        self.interests.push(interest);
    }

    pub fn remove_interest(&mut self, interest: &str) -> bool {
        if let Some(index) = self.interests.iter().position(|i| i == interest) {
            self.interests.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_interest(&self, interest: &str) -> bool {
        self.interests.contains(&String::from(interest))
    }

    pub fn list_interests(&self) -> Vec<String> {
        self.interests.clone()
    }

    pub fn count_interests(&self) -> usize {
        self.interests.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_interest_finder() {
        let mut finder = SharedInterestFinder::new();
        assert_eq!(finder.count_interests(), 0);

        finder.add_interest(String::from("reading"));
        assert_eq!(finder.count_interests(), 1);
        assert!(finder.has_interest("reading"));

        finder.add_interest(String::from("traveling"));
        assert_eq!(finder.count_interests(), 2);
        assert!(finder.has_interest("traveling"));

        assert!(finder.remove_interest("reading"));
        assert_eq!(finder.count_interests(), 1);
        assert!(!finder.has_interest("reading"));

        let interests = finder.list_interests();
        assert_eq!(interests, vec![String::from("traveling")]);

        assert!(!finder.remove_interest("hiking"));
        assert_eq!(finder.count_interests(), 1);
    }
}
