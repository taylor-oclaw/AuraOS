extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleEventRecall {
    events: Vec<(String, String)>, // (person_name, event_description)
}

impl PeopleEventRecall {
    pub fn new() -> Self {
        PeopleEventRecall {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, person_name: &str, event_description: &str) {
        let name = String::from(person_name);
        let description = String::from(event_description);
        self.events.push((name, description));
    }

    pub fn get_events_for_person(&self, person_name: &str) -> Vec<String> {
        self.events
            .iter()
            .filter(|&&(ref name, _)| name == person_name)
            .map(|(_, desc)| desc.clone())
            .collect()
    }

    pub fn remove_event(&mut self, event_description: &str) {
        self.events.retain(|(_, desc)| desc != event_description);
    }

    pub fn list_all_events(&self) -> Vec<String> {
        self.events
            .iter()
            .map(|(name, desc)| format!("{}: {}", name, desc))
            .collect()
    }

    pub fn count_events_for_person(&self, person_name: &str) -> usize {
        self.events.iter().filter(|&&(ref name, _)| name == person_name).count()
    }
}
