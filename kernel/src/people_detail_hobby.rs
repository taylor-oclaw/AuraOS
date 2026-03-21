extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: String::from(name),
            age,
            hobbies: Vec::new(),
        }
    }

    pub fn add_hobby(&mut self, hobby: &str) {
        self.hobbies.push(String::from(hobby));
    }

    pub fn remove_hobby(&mut self, hobby: &str) -> bool {
        if let Some(index) = self.hobbies.iter().position(|h| h == hobby) {
            self.hobbies.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn list_hobbies(&self) -> &[String] {
        &self.hobbies
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
        assert!(person.list_hobbies().is_empty());
    }

    #[test]
    fn test_add_and_remove_hobby() {
        let mut person = Person::new("Bob", 25);
        person.add_hobby("Reading");
        person.add_hobby("Gaming");
        assert_eq!(person.list_hobbies(), &["Reading".to_string(), "Gaming".to_string()]);
        assert!(person.remove_hobby("Reading"));
        assert_eq!(person.list_hobbies(), &["Gaming".to_string()]);
        assert!(!person.remove_hobby("Swimming"));
    }
}
