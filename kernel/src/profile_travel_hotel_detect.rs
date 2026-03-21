extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ProfileTravelHotelDetect {
    profiles: Vec<String>,
    hotels: Vec<String>,
    travel_destinations: Vec<String>,
}

impl ProfileTravelHotelDetect {
    pub fn new() -> Self {
        ProfileTravelHotelDetect {
            profiles: Vec::new(),
            hotels: Vec::new(),
            travel_destinations: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile: String) {
        self.profiles.push(profile);
    }

    pub fn add_hotel(&mut self, hotel: String) {
        self.hotels.push(hotel);
    }

    pub fn add_travel_destination(&mut self, destination: String) {
        self.travel_destinations.push(destination);
    }

    pub fn get_profiles(&self) -> &Vec<String> {
        &self.profiles
    }

    pub fn get_hotels(&self) -> &Vec<String> {
        &self.hotels
    }

    pub fn get_travel_destinations(&self) -> &Vec<String> {
        &self.travel_destinations
    }
}
