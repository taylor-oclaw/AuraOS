extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FamilyHubCalendarShare {
    events: Vec<Event>,
}

impl FamilyHubCalendarShare {
    pub fn new() -> Self {
        FamilyHubCalendarShare {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn remove_event(&mut self, event_id: u32) -> bool {
        if let Some(index) = self.events.iter().position(|e| e.id == event_id) {
            self.events.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_event_by_id(&self, event_id: u32) -> Option<&Event> {
        self.events.iter().find(|&e| e.id == event_id)
    }

    pub fn list_events(&self) -> &[Event] {
        &self.events
    }

    pub fn update_event(&mut self, event_id: u32, new_event: Event) -> bool {
        if let Some(index) = self.events.iter().position(|e| e.id == event_id) {
            self.events[index] = new_event;
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub date: String, // Assuming date is stored as a string for simplicity
    pub time: String,  // Assuming time is stored as a string for simplicity
}
