extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
    height: f32, // in meters
    weight: f32, // in kilograms
}

impl Person {
    pub fn new(name: &str, age: u32, height: f32, weight: f32) -> Self {
        Person {
            name: String::from(name),
            age,
            height,
            weight,
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

    pub fn calculate_bmi(&self) -> f32 {
        if self.height > 0.0 {
            self.weight / (self.height * self.height)
        } else {
            0.0 // Avoid division by zero
        }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_creation() {
        let person = Person::new("Alice", 30, 1.75, 65.0);
        assert_eq!(person.get_name(), "Alice");
        assert_eq!(person.get_age(), 30);
    }

    #[test]
    fn test_setters() {
        let mut person = Person::new("Bob", 25, 1.80, 70.0);
        person.set_name("Robert");
        person.set_age(26);
        assert_eq!(person.get_name(), "Robert");
        assert_eq!(person.get_age(), 26);
    }

    #[test]
    fn test_bmi_calculation() {
        let person = Person::new("Charlie", 35, 1.70, 80.0);
        let bmi = person.calculate_bmi();
        assert!((bmi - 28.98).abs() < 0.01); // Allow for floating-point precision errors
    }

    #[test]
    fn test_adult_check() {
        let adult = Person::new("David", 18, 1.75, 65.0);
        let minor = Person::new("Eve", 17, 1.60, 55.0);
        assert!(adult.is_adult());
        assert!(!minor.is_adult());
    }
}
