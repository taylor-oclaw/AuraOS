extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarTravelTime {
    events: Vec<(String, u32)>, // (event_name, travel_time_in_minutes)
}

impl CalendarTravelTime {
    pub fn new() -> Self {
        CalendarTravelTime {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event_name: String, travel_time: u32) {
        self.events.push((event_name, travel_time));
    }

    pub fn get_total_travel_time(&self) -> u32 {
        self.events.iter().map(|(_, time)| *time).sum()
    }

    pub fn find_event_by_name(&self, event_name: &str) -> Option<&(String, u32)> {
        self.events.iter().find(|&&(ref name, _)| name == event_name)
    }

    pub fn remove_event_by_name(&mut self, event_name: &str) {
        self.events.retain(|(name, _)| name != event_name);
    }

    pub fn list_events(&self) -> Vec<&String> {
        self.events.iter().map(|(event_name, _)| event_name).collect()
    }
}
