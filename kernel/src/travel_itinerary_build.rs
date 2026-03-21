extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelItinerary {
    destinations: Vec<String>,
    current_day: usize,
}

impl TravelItinerary {
    pub fn new() -> Self {
        TravelItinerary {
            destinations: Vec::new(),
            current_day: 0,
        }
    }

    pub fn add_destination(&mut self, destination: String) {
        self.destinations.push(destination);
    }

    pub fn get_current_destination(&self) -> Option<&String> {
        if self.current_day < self.destinations.len() {
            Some(&self.destinations[self.current_day])
        } else {
            None
        }
    }

    pub fn move_to_next_day(&mut self) {
        if self.current_day < self.destinations.len() {
            self.current_day += 1;
        }
    }

    pub fn get_total_days(&self) -> usize {
        self.destinations.len()
    }

    pub fn reset_itinerary(&mut self) {
        self.current_day = 0;
    }
}
