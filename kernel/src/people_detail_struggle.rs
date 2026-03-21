extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct Person {
    name: String,
    age: u8,
    occupation: String,
    skills: Vec<String>,
    is_student: bool,
}

impl Person {
    pub fn new(name: &str, age: u8, occupation: &str, skills: &[&str], is_student: bool) -> Self {
        Person {
            name: String::from(name),
            age,
            occupation: String::from(occupation),
            skills: skills.iter().map(|s| String::from(*s)).collect(),
            is_student,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn add_skill(&mut self, skill: &str) {
        self.skills.push(String::from(skill));
    }

    pub fn is_student(&self) -> bool {
        self.is_student
    }

    pub fn list_skills(&self) -> Vec<&str> {
        self.skills.iter().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_creation() {
        let person = Person::new("Alice", 30, "Engineer", &["Rust", "C++"], false);
        assert_eq!(person.get_name(), "Alice");
        assert_eq!(person.age, 30);
        assert_eq!(person.occupation, "Engineer");
        assert_eq!(person.list_skills(), vec!["Rust", "C++"]);
        assert_eq!(person.is_student(), false);
    }

    #[test]
    fn test_set_age() {
        let mut person = Person::new("Bob", 25, "Designer", &["Photoshop"], true);
        person.set_age(30);
        assert_eq!(person.age, 30);
    }

    #[test]
    fn test_add_skill() {
        let mut person = Person::new("Charlie", 40, "Teacher", &["Math"], false);
        person.add_skill("Science");
        assert_eq!(person.list_skills(), vec!["Math", "Science"]);
    }
}
