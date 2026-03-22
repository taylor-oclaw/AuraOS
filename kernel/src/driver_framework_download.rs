extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DriverFrameworkDownload {
    drivers: Vec<String>,
}

impl DriverFrameworkDownload {
    pub fn new() -> Self {
        DriverFrameworkDownload {
            drivers: Vec::new(),
        }
    }

    pub fn add_driver(&mut self, driver_name: &str) {
        self.drivers.push(String::from(driver_name));
    }

    pub fn remove_driver(&mut self, driver_name: &str) {
        if let Some(index) = self.drivers.iter().position(|d| d == driver_name) {
            self.drivers.remove(index);
        }
    }

    pub fn list_drivers(&self) -> Vec<String> {
        self.drivers.clone()
    }

    pub fn has_driver(&self, driver_name: &str) -> bool {
        self.drivers.contains(&String::from(driver_name))
    }

    pub fn count_drivers(&self) -> usize {
        self.drivers.len()
    }
}