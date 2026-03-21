extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Person {
    name: String,
    age: u32,
    interests: Vec<String>,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: String::from(name),
            age,
            interests: Vec::new(),
        }
    }

    pub fn add_interest(&mut self, interest: &str) {
        self.interests.push(String::from(interest));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn list_interests(&self) -> &Vec<String> {
        &self.interests
    }

    pub fn has_interest(&self, interest: &str) -> bool {
        self.interests.contains(&String::from(interest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_creation() {
        let person = Person::new("Alice", 30);
        assert_eq!(person.get_name(), "Alice");
        assert_eq!(person.get_age(), 30);
    }

    #[test]
    fn test_add_interest() {
        let mut person = Person::new("Bob", 25);
        person.add_interest("Programming");
        assert!(person.has_interest("Programming"));
    }

    #[test]
    fn test_list_interests() {
        let mut person = Person::new("Charlie", 35);
        person.add_interest("Reading");
        person.add_interest("Hiking");
        assert_eq!(person.list_interests().len(), 2);
    }
}
