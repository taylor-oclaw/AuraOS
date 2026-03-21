extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn car_fuel_track_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn car_fuel_track_exit() {
    // Cleanup logic for the module
}

pub struct CarFuelTracker {
    cars: Vec<String>,
    fuel_levels: Vec<u32>,
}

impl CarFuelTracker {
    pub fn new() -> Self {
        CarFuelTracker {
            cars: Vec::new(),
            fuel_levels: Vec::new(),
        }
    }

    pub fn add_car(&mut self, car_name: &str) {
        self.cars.push(String::from(car_name));
        self.fuel_levels.push(0);
    }

    pub fn set_fuel_level(&mut self, car_index: usize, fuel_level: u32) -> Result<(), &'static str> {
        if car_index < self.cars.len() {
            self.fuel_levels[car_index] = fuel_level;
            Ok(())
        } else {
            Err("Car index out of bounds")
        }
    }

    pub fn get_fuel_level(&self, car_index: usize) -> Result<u32, &'static str> {
        if car_index < self.cars.len() {
            Ok(self.fuel_levels[car_index])
        } else {
            Err("Car index out of bounds")
        }
    }

    pub fn list_cars(&self) -> Vec<&str> {
        self.cars.iter().map(|car| car.as_str()).collect()
    }

    pub fn total_fuel(&self) -> u32 {
        self.fuel_levels.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_fuel_tracker() {
        let mut tracker = CarFuelTracker::new();
        tracker.add_car("Car1");
        tracker.add_car("Car2");

        assert_eq!(tracker.set_fuel_level(0, 50), Ok(()));
        assert_eq!(tracker.set_fuel_level(1, 75), Ok(()));

        assert_eq!(tracker.get_fuel_level(0), Ok(50));
        assert_eq!(tracker.get_fuel_level(1), Ok(75));

        assert_eq!(tracker.list_cars(), vec!["Car1", "Car2"]);
        assert_eq!(tracker.total_fuel(), 125);
    }
}
