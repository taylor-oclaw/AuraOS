extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct CarParking {
    spaces: Vec<Space>,
}

impl CarParking {
    pub fn new(capacity: usize) -> Self {
        let mut spaces = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            spaces.push(Space::new());
        }
        CarParking { spaces }
    }

    pub fn park(&mut self, car_id: &str) -> Result<usize, String> {
        if let Some(index) = self.spaces.iter().position(|s| s.is_empty()) {
            self.spaces[index].occupy(car_id);
            Ok(index)
        } else {
            Err(String::from("No available spaces"))
        }
    }

    pub fn leave(&mut self, index: usize) -> Result<String, String> {
        if let Some(space) = self.spaces.get_mut(index) {
            if space.is_occupied() {
                let car_id = space.leave();
                Ok(car_id)
            } else {
                Err(String::from("Space is already empty"))
            }
        } else {
            Err(String::from("Invalid space index"))
        }
    }

    pub fn find_car(&self, car_id: &str) -> Result<usize, String> {
        if let Some(index) = self.spaces.iter().position(|s| s.is_occupied() && s.car_id == car_id) {
            Ok(index)
        } else {
            Err(String::from("Car not found"))
        }
    }

    pub fn available_spaces(&self) -> usize {
        self.spaces.iter().filter(|s| s.is_empty()).count()
    }

    pub fn occupied_spaces(&self) -> usize {
        self.spaces.iter().filter(|s| s.is_occupied()).count()
    }
}

#[derive(Debug)]
struct Space {
    car_id: String,
    is_occupied: bool,
}

impl Space {
    fn new() -> Self {
        Space {
            car_id: String::new(),
            is_occupied: false,
        }
    }

    fn occupy(&mut self, car_id: &str) {
        self.car_id = car_id.to_string();
        self.is_occupied = true;
    }

    fn leave(&mut self) -> String {
        let car_id = self.car_id.clone();
        self.car_id.clear();
        self.is_occupied = false;
        car_id
    }

    fn is_empty(&self) -> bool {
        !self.is_occupied
    }

    fn is_occupied(&self) -> bool {
        self.is_occupied
    }
}
