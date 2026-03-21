extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod people_inside_joke {
    use super::*;

    pub struct PeopleInsideJoke {
        participants: Vec<String>,
        jokes: Vec<String>,
    }

    impl PeopleInsideJoke {
        pub fn new() -> Self {
            PeopleInsideJoke {
                participants: Vec::new(),
                jokes: Vec::new(),
            }
        }

        pub fn add_participant(&mut self, name: &str) {
            self.participants.push(String::from(name));
        }

        pub fn remove_participant(&mut self, name: &str) -> bool {
            if let Some(index) = self.participants.iter().position(|n| n == name) {
                self.participants.remove(index);
                true
            } else {
                false
            }
        }

        pub fn add_joke(&mut self, joke: &str) {
            self.jokes.push(String::from(joke));
        }

        pub fn get_random_joke(&self) -> Option<&String> {
            if !self.jokes.is_empty() {
                Some(&self.jokes[0]) // Simplified to return the first joke for no_std compatibility
            } else {
                None
            }
        }

        pub fn list_participants(&self) -> Vec<String> {
            self.participants.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::people_inside_joke::*;

    #[test]
    fn test_people_inside_joke() {
        let mut joke_module = PeopleInsideJoke::new();
        joke_module.add_participant("Alice");
        joke_module.add_participant("Bob");

        assert_eq!(joke_module.list_participants(), vec![String::from("Alice"), String::from("Bob")]);
        assert!(joke_module.remove_participant("Alice"));
        assert_eq!(joke_module.list_participants(), vec![String::from("Bob")]);

        joke_module.add_joke("Why was the math book sad?");
        assert_eq!(joke_module.get_random_joke(), Some(&String::from("Why was the math book sad?")));
    }
}
