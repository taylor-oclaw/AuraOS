extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SeatTracker {
    seats: Vec<String>,
}

impl SeatTracker {
    pub fn new() -> Self {
        SeatTracker { seats: Vec::new() }
    }

    pub fn add_seat(&mut self, seat_name: String) {
        if !self.seats.contains(&seat_name) {
            self.seats.push(seat_name);
        }
    }

    pub fn remove_seat(&mut self, seat_name: &String) -> bool {
        let index = self.seats.iter().position(|s| s == seat_name);
        match index {
            Some(i) => {
                self.seats.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn get_seats(&self) -> Vec<String> {
        self.seats.clone()
    }

    pub fn count_seats(&self) -> usize {
        self.seats.len()
    }

    pub fn is_seat_taken(&self, seat_name: &String) -> bool {
        self.seats.contains(seat_name)
    }
}