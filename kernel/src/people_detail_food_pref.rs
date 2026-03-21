extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
    favorite_foods: Vec<String>,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Self {
        Person {
            name: String::from(name),
            age,
            favorite_foods: Vec::new(),
        }
    }

    pub fn add_favorite_food(&mut self, food: &str) {
        self.favorite_foods.push(String::from(food));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn list_favorite_foods(&self) -> &[String] {
        &self.favorite_foods
    }

    pub fn has_favorite_food(&self, food: &str) -> bool {
        self.favorite_foods.iter().any(|f| f == food)
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
        assert!(person.list_favorite_foods().is_empty());
    }

    #[test]
    fn test_add_favorite_food() {
        let mut person = Person::new("Bob", 25);
        person.add_favorite_food("Pizza");
        person.add_favorite_food("Burger");
        assert_eq!(person.list_favorite_foods(), vec!["Pizza", "Burger"]);
    }

    #[test]
    fn test_has_favorite_food() {
        let mut person = Person::new("Charlie", 35);
        person.add_favorite_food("Sushi");
        assert!(person.has_favorite_food("Sushi"));
        assert!(!person.has_favorite_food("Pasta"));
    }
}
