extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct TravelModePredictor {
    // Example data structure for simplicity
    travel_modes: Vec<String>,
}

impl TravelModePredictor {
    pub fn new() -> Self {
        TravelModePredictor {
            travel_modes: vec![
                String::from("Walking"),
                String::from("Biking"),
                String::from("Car"),
                String::from("Public Transport"),
                String::from("Flying"),
            ],
        }
    }

    pub fn add_travel_mode(&mut self, mode: &str) {
        if !self.travel_modes.contains(&String::from(mode)) {
            self.travel_modes.push(String::from(mode));
        }
    }

    pub fn remove_travel_mode(&mut self, mode: &str) {
        self.travel_modes.retain(|m| m != mode);
    }

    pub fn list_travel_modes(&self) -> Vec<String> {
        self.travel_modes.clone()
    }

    pub fn predict_mode(&self, context: &str) -> Option<&String> {
        // Simple prediction logic based on context
        if context.contains("bike") || context.contains("Biking") {
            self.travel_modes.iter().find(|&m| m == "Biking")
        } else if context.contains("car") || context.contains("Car") {
            self.travel_modes.iter().find(|&m| m == "Car")
        } else if context.contains("train") || context.contains("Train") {
            self.travel_modes.iter().find(|&m| m == "Public Transport")
        } else if context.contains("plane") || context.contains("Plane") {
            self.travel_modes.iter().find(|&m| m == "Flying")
        } else {
            Some(&self.travel_modes[0]) // Default to first mode
        }
    }

    pub fn count_travel_modes(&self) -> usize {
        self.travel_modes.len()
    }
}
