extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_per_person_learn_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_per_person_learn_exit() {
    // Cleanup logic for the module
}

pub struct TonePerPersonLearn {
    data: Vec<(String, String)>,
}

impl TonePerPersonLearn {
    pub fn new() -> Self {
        TonePerPersonLearn { data: Vec::new() }
    }

    pub fn add_person(&mut self, name: &str, tone: &str) {
        let name = String::from(name);
        let tone = String::from(tone);
        self.data.push((name, tone));
    }

    pub fn get_tone(&self, name: &str) -> Option<&String> {
        for (n, t) in &self.data {
            if n == name {
                return Some(t);
            }
        }
        None
    }

    pub fn remove_person(&mut self, name: &str) {
        self.data.retain(|(n, _)| n != name);
    }

    pub fn list_people(&self) -> Vec<&String> {
        self.data.iter().map(|(n, _)| n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_per_person_learn() {
        let mut tpp = TonePerPersonLearn::new();
        tpp.add_person("Alice", "Happy");
        tpp.add_person("Bob", "Sad");

        assert_eq!(tpp.get_tone("Alice"), Some(&String::from("Happy")));
        assert_eq!(tpp.get_tone("Bob"), Some(&String::from("Sad")));

        tpp.remove_person("Alice");
        assert_eq!(tpp.get_tone("Alice"), None);

        let people = tpp.list_people();
        assert_eq!(people.len(), 1);
        assert_eq!(people[0], &String::from("Bob"));
    }
}
