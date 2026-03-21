extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rel_seating_arrange_init() {
}

pub extern "C" fn rel_seating_arrange_exit() {
}

pub struct SeatingArrangement {
    seats: Vec<String>,
}

impl SeatingArrangement {
    pub fn new(capacity: usize) -> Self {
        SeatingArrangement {
            seats: vec![String::new(); capacity],
        }
    }

    pub fn assign_seat(&mut self, seat_number: usize, person_name: &str) -> Result<(), &'static str> {
        if seat_number >= self.seats.len() {
            Err("Seat number out of bounds")
        } else {
            self.seats[seat_number] = String::from(person_name);
            Ok(())
        }
    }

    pub fn get_seat(&self, seat_number: usize) -> Result<&str, &'static str> {
        if seat_number >= self.seats.len() {
            Err("Seat number out of bounds")
        } else {
            Ok(self.seats[seat_number].as_str())
        }
    }

    pub fn is_seat_taken(&self, seat_number: usize) -> Result<bool, &'static str> {
        if seat_number >= self.seats.len() {
            Err("Seat number out of bounds")
        } else {
            Ok(!self.seats[seat_number].is_empty())
        }
    }

    pub fn list_seats(&self) -> Vec<&str> {
        self.seats.iter().map(|s| s.as_str()).collect()
    }
}
