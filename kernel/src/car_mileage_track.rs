extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut tracker = CarMileageTracker::new();
    tracker.add_car("Toyota Camry".into(), 15000);
    tracker.add_car("Honda Accord".into(), 20000);
    tracker.update_mileage("Toyota Camry", 16000);
    let mileage = tracker.get_mileage("Honda Accord");
    let total_miles = tracker.total_miles();

    loop {}
}

pub struct CarMileageTracker {
    cars: Vec<(String, u32)>,
}

impl CarMileageTracker {
    pub fn new() -> Self {
        CarMileageTracker { cars: Vec::new() }
    }

    pub fn add_car(&mut self, name: String, mileage: u32) {
        self.cars.push((name, mileage));
    }

    pub fn update_mileage(&mut self, car_name: &str, new_mileage: u32) {
        if let Some(car) = self.cars.iter_mut().find(|(name, _)| name == car_name) {
            car.1 = new_mileage;
        }
    }

    pub fn get_mileage(&self, car_name: &str) -> Option<u32> {
        self.cars.iter().find_map(|(name, mileage)| {
            if name == car_name {
                Some(*mileage)
            } else {
                None
            }
        }
    }

    pub fn total_miles(&self) -> u32 {
        self.cars.iter().map(|(_, mileage)| *mileage).sum()
    }

    pub fn list_cars(&self) -> Vec<String> {
        self.cars.iter().map(|(name, _)| name.clone()).collect()
    }
}
