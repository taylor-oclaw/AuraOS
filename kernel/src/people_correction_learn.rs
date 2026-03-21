extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct Person {
    name: String,
    age: u8,
    skills: Vec<String>,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Self {
        Person {
            name: String::from(name),
            age,
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn list_skills(&self) -> &[String] {
        &self.skills
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.contains(&String::from(skill))
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
    fn test_add_skill() {
        let mut person = Person::new("Bob", 25);
        person.add_skill("Rust");
        assert!(person.has_skill("Rust"));
    }

    #[test]
    fn test_list_skills() {
        let mut person = Person::new("Charlie", 35);
        person.add_skill("AI");
        person.add_skill("Machine Learning");
        assert_eq!(person.list_skills().len(), 2);
    }
}
