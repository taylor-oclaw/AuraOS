extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct CulturalContext {
    people: Vec<String>,
    culture: String,
}

impl CulturalContext {
    pub fn new(people: Vec<String>, culture: String) -> Self {
        CulturalContext { people, culture }
    }

    pub fn add_person(&mut self, person: String) {
        self.people.push(person);
    }

    pub fn remove_person(&mut self, person: &str) -> bool {
        if let Some(index) = self.people.iter().position(|p| p == person) {
            self.people.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_culture(&self) -> &String {
        &self.culture
    }

    pub fn set_culture(&mut self, culture: String) {
        self.culture = culture;
    }

    pub fn list_people(&self) -> &Vec<String> {
        &self.people
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cultural_context() {
        let mut context = CulturalContext::new(vec![String::from("Alice"), String::from("Bob")], String::from("Tech Culture"));

        assert_eq!(context.list_people().len(), 2);
        assert_eq!(context.get_culture(), &String::from("Tech Culture"));

        context.add_person(String::from("Charlie"));
        assert_eq!(context.list_people().len(), 3);

        assert!(context.remove_person("Bob"));
        assert_eq!(context.list_people().len(), 2);
        assert!(!context.remove_person("David"));

        context.set_culture(String::from("Innovation Culture"));
        assert_eq!(context.get_culture(), &String::from("Innovation Culture"));
    }
}
