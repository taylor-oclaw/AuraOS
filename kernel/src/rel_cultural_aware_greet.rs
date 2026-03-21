extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct CulturalGreeting {
    greetings: Vec<String>,
}

impl CulturalGreeting {
    pub fn new() -> Self {
        CulturalGreeting {
            greetings: vec![
                String::from("Hello"),
                String::from("Bonjour"),
                String::from("Hola"),
                String::from("Ciao"),
                String::from("Namaste"),
            ],
        }
    }

    pub fn add_greeting(&mut self, greeting: &str) {
        self.greetings.push(String::from(greeting));
    }

    pub fn remove_greeting(&mut self, index: usize) -> Option<String> {
        if index < self.greetings.len() {
            Some(self.greetings.remove(index))
        } else {
            None
        }
    }

    pub fn get_greeting(&self, index: usize) -> Option<&String> {
        self.greetings.get(index)
    }

    pub fn list_greetings(&self) -> &[String] {
        &self.greetings
    }

    pub fn count_greetings(&self) -> usize {
        self.greetings.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cultural_greeting() {
        let mut cg = CulturalGreeting::new();
        assert_eq!(cg.count_greetings(), 5);
        assert_eq!(cg.get_greeting(0), Some(&String::from("Hello")));
        assert_eq!(cg.get_greeting(10), None);

        cg.add_greeting("Zdravo");
        assert_eq!(cg.count_greetings(), 6);
        assert_eq!(cg.get_greeting(5), Some(&String::from("Zdravo")));

        let removed = cg.remove_greeting(2);
        assert_eq!(removed, Some(String::from("Hola")));
        assert_eq!(cg.count_greetings(), 5);

        let greetings = cg.list_greetings();
        assert_eq!(greetings.len(), 5);
    }
}
