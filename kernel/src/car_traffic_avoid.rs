extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CarTrafficAvoid {
    cars: Vec<String>,
    obstacles: Vec<String>,
}

impl CarTrafficAvoid {
    pub fn new() -> Self {
        CarTrafficAvoid {
            cars: Vec::new(),
            obstacles: Vec::new(),
        }
    }

    pub fn add_car(&mut self, car_id: &str) {
        self.cars.push(car_id.to_string());
    }

    pub fn remove_car(&mut self, car_id: &str) -> bool {
        if let Some(index) = self.cars.iter().position(|c| c == car_id) {
            self.cars.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_obstacle(&mut self, obstacle_id: &str) {
        self.obstacles.push(obstacle_id.to_string());
    }

    pub fn remove_obstacle(&mut self, obstacle_id: &str) -> bool {
        if let Some(index) = self.obstacles.iter().position(|o| o == obstacle_id) {
            self.obstacles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn check_traffic(&self) -> String {
        let mut status = String::from("Traffic Status:\n");
        if self.cars.is_empty() {
            status.push_str("No cars present.\n");
        } else {
            status.push_str("Cars present: ");
            for car in &self.cars {
                status.push_str(car);
                status.push(',');
            }
            status.pop(); // Remove the last comma
            status.push('\n');
        }

        if self.obstacles.is_empty() {
            status.push_str("No obstacles present.\n");
        } else {
            status.push_str("Obstacles present: ");
            for obstacle in &self.obstacles {
                status.push_str(obstacle);
                status.push(',');
            }
            status.pop(); // Remove the last comma
            status.push('\n');
        }

        status
    }
}
