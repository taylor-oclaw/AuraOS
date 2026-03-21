extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct FlightTrack {
    flight_id: String,
    destination: String,
    passengers: Vec<String>,
    altitude: u32,
    speed: u32,
}

impl FlightTrack {
    pub fn new(flight_id: &str, destination: &str) -> Self {
        FlightTrack {
            flight_id: String::from(flight_id),
            destination: String::from(destination),
            passengers: Vec::new(),
            altitude: 0,
            speed: 0,
        }
    }

    pub fn add_passenger(&mut self, passenger_name: &str) {
        self.passengers.push(String::from(passenger_name));
    }

    pub fn remove_passenger(&mut self, passenger_name: &str) -> bool {
        if let Some(index) = self.passengers.iter().position(|p| p == passenger_name) {
            self.passengers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_altitude(&mut self, altitude: u32) {
        self.altitude = altitude;
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn get_flight_info(&self) -> String {
        let mut info = String::from("info");
        info.push_str(String::from("info").as_str());
        info.push_str(String::from("info").as_str());
        info.push_str(String::from("info").as_str());
        info.push_str(String::from("info").as_str());
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flight_track() {
        let mut flight = FlightTrack::new("Flight123", "New York");
        assert_eq!(flight.flight_id, "Flight123");
        assert_eq!(flight.destination, "New York");

        flight.add_passenger("Alice");
        flight.add_passenger("Bob");
        assert_eq!(flight.passengers.len(), 2);

        let removed = flight.remove_passenger("Alice");
        assert!(removed);
        assert_eq!(flight.passengers.len(), 1);

        flight.set_altitude(30000);
        flight.set_speed(800);
        assert_eq!(flight.altitude, 30000);
        assert_eq!(flight.speed, 800);

        let info = flight.get_flight_info();
        assert!(info.contains("Flight ID: Flight123"));
        assert!(info.contains("Destination: New York"));
        assert!(info.contains("Passengers: [\"Bob\"]"));
        assert!(info.contains("Altitude: 30000 meters"));
        assert!(info.contains("Speed: 800 km/h"));
    }
}
