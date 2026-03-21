extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn people_voice_remember_init() {
    // Initialization logic for the module
}

pub extern "C" fn people_voice_remember_exit() {
    // Cleanup logic for the module
}

pub struct PeopleVoiceRemember {
    names: Vec<String>,
    voices: Vec<Vec<u8>>,
}

impl PeopleVoiceRemember {
    pub fn new() -> Self {
        PeopleVoiceRemember {
            names: Vec::new(),
            voices: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String, voice: Vec<u8>) {
        self.names.push(name);
        self.voices.push(voice);
    }

    pub fn get_voice_by_name(&self, name: &str) -> Option<&Vec<u8>> {
        self.names.iter().position(|n| n == name).map(|index| &self.voices[index])
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        if let Some(index) = self.names.iter().position(|n| n == name) {
            self.names.remove(index);
            self.voices.remove(index);
        }
    }

    pub fn list_people(&self) -> Vec<&String> {
        self.names.iter().collect()
    }

    pub fn count_people(&self) -> usize {
        self.names.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_person() {
        let mut pvr = PeopleVoiceRemember::new();
        let name = String::from("Alice");
        let voice = vec![1, 2, 3];
        pvr.add_person(name.clone(), voice.clone());

        assert_eq!(pvr.get_voice_by_name(&name), Some(&voice));
    }

    #[test]
    fn test_remove_person() {
        let mut pvr = PeopleVoiceRemember::new();
        let name = String::from("Bob");
        let voice = vec![4, 5, 6];
        pvr.add_person(name.clone(), voice);

        pvr.remove_person_by_name(&name);
        assert_eq!(pvr.get_voice_by_name(&name), None);
    }

    #[test]
    fn test_list_people() {
        let mut pvr = PeopleVoiceRemember::new();
        pvr.add_person(String::from("Charlie"), vec![7, 8, 9]);
        pvr.add_person(String::from("David"), vec![10, 11, 12]);

        let people = pvr.list_people();
        assert_eq!(people.len(), 2);
        assert!(people.contains(&&String::from("Charlie")));
        assert!(people.contains(&&String::from("David")));
    }

    #[test]
    fn test_count_people() {
        let mut pvr = PeopleVoiceRemember::new();
        assert_eq!(pvr.count_people(), 0);

        pvr.add_person(String::from("Eve"), vec![13, 14, 15]);
        assert_eq!(pvr.count_people(), 1);
    }
}
