extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn notify_night_owl_detect_init() {
    // Initialization logic for the module
}

pub extern "C" fn notify_night_owl_detect_exit() {
    // Cleanup logic for the module
}

pub struct NightOwlDetector {
    detected_animals: Vec<String>,
    threshold: u8,
}

impl NightOwlDetector {
    pub fn new(threshold: u8) -> Self {
        NightOwlDetector {
            detected_animals: Vec::new(),
            threshold,
        }
    }

    pub fn add_detected_animal(&mut self, animal_name: &str) {
        if !self.detected_animals.contains(&animal_name.to_string()) {
            self.detected_animals.push(animal_name.to_string());
        }
    }

    pub fn remove_detected_animal(&mut self, animal_name: &str) {
        self.detected_animals.retain(|name| name != animal_name);
    }

    pub fn get_detected_animals(&self) -> &[String] {
        &self.detected_animals
    }

    pub fn is_night_owl_detected(&self) -> bool {
        self.detected_animals.iter().any(|animal| animal == "Night Owl")
    }

    pub fn set_threshold(&mut self, threshold: u8) {
        self.threshold = threshold;
    }
}
