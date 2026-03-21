extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelHotelSuggest {
    hotels: Vec<String>,
}

impl TravelHotelSuggest {
    pub fn new() -> Self {
        TravelHotelSuggest {
            hotels: Vec::new(),
        }
    }

    pub fn add_hotel(&mut self, hotel_name: &str) {
        self.hotels.push(String::from(hotel_name));
    }

    pub fn remove_hotel(&mut self, hotel_name: &str) -> bool {
        if let Some(index) = self.hotels.iter().position(|h| h == hotel_name) {
            self.hotels.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_hotels(&self) -> Vec<String> {
        self.hotels.clone()
    }

    pub fn find_hotel(&self, hotel_name: &str) -> Option<&String> {
        self.hotels.iter().find(|&&h| h == hotel_name)
    }

    pub fn count_hotels(&self) -> usize {
        self.hotels.len()
    }
}
