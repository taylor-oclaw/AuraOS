extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct CarEvChargeFind {
    stations: Vec<Station>,
}

impl CarEvChargeFind {
    pub fn new() -> Self {
        CarEvChargeFind {
            stations: Vec::new(),
        }
    }

    pub fn add_station(&mut self, station: Station) {
        self.stations.push(station);
    }

    pub fn find_nearest_station(&self, location: Location) -> Option<&Station> {
        self.stations.iter().min_by_key(|s| s.location.distance_to(location))
    }

    pub fn list_stations_in_radius(&self, location: Location, radius: u32) -> Vec<&Station> {
        self.stations
            .iter()
            .filter(|s| s.location.distance_to(location) <= radius)
            .collect()
    }

    pub fn get_station_by_id(&self, id: &str) -> Option<&Station> {
        self.stations.iter().find(|s| s.id == id)
    }

    pub fn remove_station(&mut self, id: &str) {
        self.stations.retain(|s| s.id != id);
    }
}

#[derive(Debug)]
pub struct Station {
    id: String,
    name: String,
    location: Location,
    charging_speed: u32, // in kW
    available_slots: u32,
}

impl Station {
    pub fn new(id: String, name: String, location: Location, charging_speed: u32, available_slots: u32) -> Self {
        Station {
            id,
            name,
            location,
            charging_speed,
            available_slots,
        }
    }

    pub fn is_full(&self) -> bool {
        self.available_slots == 0
    }

    pub fn has_available_slot(&self) -> bool {
        self.available_slots > 0
    }

    pub fn reserve_slot(&mut self) {
        if self.has_available_slot() {
            self.available_slots -= 1;
        }
    }

    pub fn release_slot(&mut self) {
        self.available_slots += 1;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
    x: u32,
    y: u32,
}

impl Location {
    pub fn new(x: u32, y: u32) -> Self {
        Location { x, y }
    }

    pub fn distance_to(&self, other: Location) -> u32 {
        ((self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()) as u32
    }
}
