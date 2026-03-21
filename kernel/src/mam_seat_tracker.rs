extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mam_seat_tracker_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mam_seat_tracker_exit() {
    // Cleanup logic for the module
}

pub struct SeatTracker {
    seats: Vec<String>,
    occupied_seats: usize,
}

impl SeatTracker {
    pub fn new(capacity: usize) -> Self {
        let mut seats = Vec::with_capacity(capacity);
        for i in 0..capacity {
            seats.push(format!("Seat{}", i + 1));
        }
        SeatTracker {
            seats,
            occupied_seats: 0,
        }
    }

    pub fn reserve_seat(&mut self) -> Option<String> {
        if self.occupied_seats < self.seats.len() {
            let seat = self.seats[self.occupied_seats].clone();
            self.occupied_seats += 1;
            Some(seat)
        } else {
            None
        }
    }

    pub fn release_seat(&mut self, seat: &str) -> bool {
        if let Some(index) = self.seats.iter().position(|s| s == seat) {
            if index < self.occupied_seats {
                self.seats.swap(index, self.occupied_seats - 1);
                self.occupied_seats -= 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_seat_occupied(&self, seat: &str) -> bool {
        self.seats.iter().take(self.occupied_seats).any(|s| s == seat)
    }

    pub fn available_seats_count(&self) -> usize {
        self.seats.len() - self.occupied_seats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_tracker() {
        let mut tracker = SeatTracker::new(3);
        assert_eq!(tracker.available_seats_count(), 3);

        let seat1 = tracker.reserve_seat().unwrap();
        assert_eq!(seat1, "Seat1");
        assert_eq!(tracker.available_seats_count(), 2);

        let seat2 = tracker.reserve_seat().unwrap();
        assert_eq!(seat2, "Seat2");
        assert_eq!(tracker.available_seats_count(), 1);

        assert!(!tracker.is_seat_occupied("Seat3"));
        assert!(tracker.release_seat(&seat1));
        assert!(tracker.is_seat_occupied("Seat2"));
        assert_eq!(tracker.available_seats_count(), 2);
    }
}
