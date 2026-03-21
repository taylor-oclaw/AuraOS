extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn car_connect_init() {
    // Initialization logic for the module
}

pub extern "C" fn car_connect_exit() {
    // Cleanup logic for the module
}

pub struct CarConnect {
    connected_cars: Vec<String>,
    max_connections: usize,
}

impl CarConnect {
    pub fn new(max_connections: usize) -> Self {
        CarConnect {
            connected_cars: Vec::new(),
            max_connections,
        }
    }

    pub fn connect(&mut self, car_id: &str) -> bool {
        if self.connected_cars.len() < self.max_connections {
            self.connected_cars.push(car_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn disconnect(&mut self, car_id: &str) -> bool {
        let pos = self.connected_cars.iter().position(|id| id == car_id);
        if let Some(index) = pos {
            self.connected_cars.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_connected(&self, car_id: &str) -> bool {
        self.connected_cars.contains(&car_id.to_string())
    }

    pub fn list_connections(&self) -> Vec<String> {
        self.connected_cars.clone()
    }

    pub fn connection_count(&self) -> usize {
        self.connected_cars.len()
    }
}
