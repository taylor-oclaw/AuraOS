extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod people_story_recall {
    use core::fmt::{Debug, Formatter};
    use alloc::boxed::Box;

    pub struct Person {
        name: String,
        age: u32,
        stories: Vec<String>,
    }

    impl Person {
        pub fn new(name: &str, age: u32) -> Self {
            Person {
                name: String::from(name),
                age,
                stories: Vec::new(),
            }
        }

        pub fn add_story(&mut self, story: &str) {
            self.stories.push(String::from(story));
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_age(&self) -> u32 {
            self.age
        }

        pub fn get_stories(&self) -> &[String] {
            &self.stories
        }

        pub fn has_story_with_keyword(&self, keyword: &str) -> bool {
            self.stories.iter().any(|story| story.contains(keyword))
        }
    }

    impl Debug for Person {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
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
            assert!(person.get_stories().is_empty());
        }

        #[test]
        fn test_add_story() {
            let mut person = Person::new("Bob", 25);
            person.add_story("Bob went to the market.");
            assert_eq!(person.get_stories().len(), 1);
            assert_eq!(person.get_stories()[0], "Bob went to the market.");
        }

        #[test]
        fn test_has_story_with_keyword() {
            let mut person = Person::new("Charlie", 35);
            person.add_story("Charlie learned Rust programming.");
            person.add_story("Charlie enjoys hiking in the mountains.");
            assert!(person.has_story_with_keyword("Rust"));
            assert!(!person.has_story_with_keyword("swimming"));
        }
    }
}
