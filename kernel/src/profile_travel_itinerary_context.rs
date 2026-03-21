extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileTravelItineraryContext {
    profile_id: u32,
    itinerary_name: String,
    destinations: Vec<String>,
    travel_dates: Vec<(u32, u32)>, // (start_year, end_year)
    notes: String,
}

impl ProfileTravelItineraryContext {
    pub fn new(profile_id: u32, itinerary_name: &str) -> Self {
        ProfileTravelItineraryContext {
            profile_id,
            itinerary_name: String::from(itinerary_name),
            destinations: Vec::new(),
            travel_dates: Vec::new(),
            notes: String::new(),
        }
    }

    pub fn add_destination(&mut self, destination: &str) {
        self.destinations.push(String::from(destination));
    }

    pub fn set_travel_dates(&mut self, start_year: u32, end_year: u32) {
        self.travel_dates.push((start_year, end_year));
    }

    pub fn add_notes(&mut self, note: &str) {
        if !self.notes.is_empty() {
            self.notes.push_str(", ");
        }
        self.notes.push_str(note);
    }

    pub fn get_profile_id(&self) -> u32 {
        self.profile_id
    }

    pub fn get_itinerary_summary(&self) -> String {
        let mut summary = format!("Profile ID: {}, Itinerary Name: {}", self.profile_id, self.itinerary_name);
        if !self.destinations.is_empty() {
            summary.push_str(", Destinations: ");
            for (i, dest) in self.destinations.iter().enumerate() {
                if i > 0 {
                    summary.push_str(", ");
                }
                summary.push_str(dest);
            }
        }
        if !self.travel_dates.is_empty() {
            summary.push_str(", Travel Dates: ");
            for (i, &(start, end)) in self.travel_dates.iter().enumerate() {
                if i > 0 {
                    summary.push_str(", ");
                }
                summary.push_str(&format!("{}-{}", start, end));
            }
        }
        if !self.notes.is_empty() {
            summary.push_str(", Notes: ");
            summary.push_str(&self.notes);
        }
        summary
    }
}