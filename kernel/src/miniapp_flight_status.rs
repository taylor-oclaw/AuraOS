extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct FlightStatus {
    flight_id: String,
    status: String,
    altitude: u32,
    speed: u32,
    heading: u16,
}

impl FlightStatus {
    pub fn new(flight_id: &str, status: &str, altitude: u32, speed: u32, heading: u16) -> Self {
        FlightStatus {
            flight_id: String::from(flight_id),
            status: String::from(status),
            altitude,
            speed,
            heading,
        }
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
    }

    pub fn set_altitude(&mut self, new_altitude: u32) {
        self.altitude = new_altitude;
    }

    pub fn increase_speed(&mut self, increment: u32) {
        self.speed += increment;
    }

    pub fn decrease_speed(&mut self, decrement: u32) {
        if decrement < self.speed {
            self.speed -= decrement;
        } else {
            self.speed = 0;
        }
    }

    pub fn change_heading(&mut self, new_heading: u16) {
        self.heading = new_heading % 360; // Ensure heading is within 0-359
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flight_status() {
        let mut flight = FlightStatus::new("Flight123", "On Ground", 0, 0, 0);

        assert_eq!(flight.flight_id, "Flight123");
        assert_eq!(flight.status, "On Ground");
        assert_eq!(flight.altitude, 0);
        assert_eq!(flight.speed, 0);
        assert_eq!(flight.heading, 0);

        flight.update_status("In Air");
        flight.set_altitude(30000);
        flight.increase_speed(500);
        flight.decrease_speed(100);
        flight.change_heading(45);

        assert_eq!(flight.status, "In Air");
        assert_eq!(flight.altitude, 30000);
        assert_eq!(flight.speed, 400);
        assert_eq!(flight.heading, 45);
    }
}
