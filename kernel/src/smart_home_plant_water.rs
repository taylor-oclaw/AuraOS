extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct PlantWateringSystem {
    plants: Vec<String>,
    water_levels: Vec<u8>,
    last_watered: Vec<u32>, // Unix timestamp in seconds
}

impl PlantWateringSystem {
    pub fn new(plants: Vec<String>) -> Self {
        let mut water_levels = Vec::new();
        let mut last_watered = Vec::new();

        for _ in 0..plants.len() {
            water_levels.push(100); // Initial water level at 100%
            last_watered.push(0);   // No watering yet
        }

        PlantWateringSystem {
            plants,
            water_levels,
            last_watered,
        }
    }

    pub fn add_plant(&mut self, plant_name: String) {
        self.plants.push(plant_name);
        self.water_levels.push(100); // New plant starts with full water
        self.last_watered.push(0);   // No watering yet
    }

    pub fn remove_plant(&mut self, index: usize) {
        if index < self.plants.len() {
            self.plants.remove(index);
            self.water_levels.remove(index);
            self.last_watered.remove(index);
        }
    }

    pub fn water_plant(&mut self, index: usize, amount: u8) -> bool {
        if index < self.plants.len() && amount > 0 {
            let new_level = self.water_levels[index].saturating_sub(amount);
            self.water_levels[index] = new_level;
            self.last_watered[index] = current_timestamp();
            true
        } else {
            false
        }
    }

    pub fn check_water_level(&self, index: usize) -> Option<u8> {
        if index < self.plants.len() {
            Some(self.water_levels[index])
        } else {
            None
        }
    }

    pub fn get_plant_names(&self) -> Vec<String> {
        self.plants.clone()
    }
}

fn current_timestamp() -> u32 {
    // Placeholder for getting the current Unix timestamp
    1672531200 // Example timestamp (January 1, 2023)
}
