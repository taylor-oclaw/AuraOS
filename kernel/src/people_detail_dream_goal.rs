extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
    dream: String,
    goal: String,
}

impl Person {
    pub fn new(name: &str, age: u32, dream: &str, goal: &str) -> Self {
        Person {
            name: String::from(name),
            age,
            dream: String::from(dream),
            goal: String::from(goal),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn get_dream(&self) -> &str {
        &self.dream
    }

    pub fn set_dream(&mut self, new_dream: &str) {
        self.dream = String::from(new_dream);
    }

    pub fn get_goal(&self) -> &str {
        &self.goal
    }

    pub fn set_goal(&mut self, new_goal: &str) {
        self.goal = String::from(new_goal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_creation() {
        let person = Person::new("Alice", 30, "Travel the world", "Learn Rust");
        assert_eq!(person.get_name(), "Alice");
        assert_eq!(person.get_age(), 30);
        assert_eq!(person.get_dream(), "Travel the world");
        assert_eq!(person.get_goal(), "Learn Rust");
    }

    #[test]
    fn test_person_modification() {
        let mut person = Person::new("Bob", 25, "Write a book", "Publish a novel");
        person.set_name("Robert");
        person.set_age(26);
        person.set_dream("Run a marathon");
        person.set_goal("Complete a 10k race");

        assert_eq!(person.get_name(), "Robert");
        assert_eq!(person.get_age(), 26);
        assert_eq!(person.get_dream(), "Run a marathon");
        assert_eq!(person.get_goal(), "Complete a 10k race");
    }
}
