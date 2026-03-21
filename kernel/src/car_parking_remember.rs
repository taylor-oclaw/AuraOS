extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CarParkingRemember {
    parked_cars: Vec<String>,
}

impl CarParkingRemember {
    pub fn new() -> Self {
        CarParkingRemember {
            parked_cars: Vec::new(),
        }
    }

    pub fn park_car(&mut self, car_id: &str) {
        if !self.parked_cars.contains(&car_id.to_string()) {
            self.parked_cars.push(car_id.to_string());
        }
    }

    pub fn unpark_car(&mut self, car_id: &str) {
        if let Some(index) = self.parked_cars.iter().position(|id| id == car_id) {
            self.parked_cars.remove(index);
        }
    }

    pub fn is_parked(&self, car_id: &str) -> bool {
        self.parked_cars.contains(&car_id.to_string())
    }

    pub fn list_parked_cars(&self) -> Vec<String> {
        self.parked_cars.clone()
    }

    pub fn count_parked_cars(&self) -> usize {
        self.parked_cars.len()
    }
}
