extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct PeopleContextTimeOfYear {
    people: Vec<String>,
    current_time_of_year: String,
}

impl PeopleContextTimeOfYear {
    pub fn new() -> Self {
        PeopleContextTimeOfYear {
            people: Vec::new(),
            current_time_of_year: String::from("Spring"),
        }
    }

    pub fn add_person(&mut self, name: &str) {
        self.people.push(String::from(name));
    }

    pub fn remove_person(&mut self, name: &str) -> bool {
        if let Some(index) = self.people.iter().position(|x| x == name) {
            self.people.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn set_time_of_year(&mut self, time_of_year: &str) {
        self.current_time_of_year = String::from(time_of_year);
    }

    pub fn get_current_time_of_year(&self) -> &str {
        &self.current_time_of_year
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_context_time_of_year() {
        let mut context = PeopleContextTimeOfYear::new();
        assert_eq!(context.get_people_count(), 0);
        assert_eq!(context.get_current_time_of_year(), "Spring");

        context.add_person("Alice");
        context.add_person("Bob");
        assert_eq!(context.get_people_count(), 2);

        assert!(context.remove_person("Alice"));
        assert!(!context.remove_person("Charlie"));
        assert_eq!(context.get_people_count(), 1);

        context.set_time_of_year("Summer");
        assert_eq!(context.get_current_time_of_year(), "Summer");
    }
}