extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct KidActivity {
    name: String,
    age: u8,
    activities: Vec<String>,
}

impl KidActivity {
    pub fn new(name: &str, age: u8) -> Self {
        KidActivity {
            name: String::from(name),
            age,
            activities: Vec::new(),
        }
    }

    pub fn add_activity(&mut self, activity: &str) {
        self.activities.push(String::from(activity));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn list_activities(&self) -> &[String] {
        &self.activities
    }

    pub fn has_activity(&self, activity: &str) -> bool {
        self.activities.contains(&String::from(activity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kid_activity() {
        let mut kid = KidActivity::new("Alice", 8);
        assert_eq!(kid.get_name(), "Alice");
        assert_eq!(kid.get_age(), 8);
        assert!(kid.list_activities().is_empty());

        kid.add_activity("Reading");
        kid.add_activity("Swimming");

        assert!(!kid.list_activities().is_empty());
        assert_eq!(kid.list_activities().len(), 2);
        assert!(kid.has_activity("Reading"));
        assert!(kid.has_activity("Swimming"));
        assert!(!kid.has_activity("Cycling"));
    }
}
