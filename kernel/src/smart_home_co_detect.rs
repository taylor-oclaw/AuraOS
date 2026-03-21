extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut co_detector = SmartHomeCODetect::new();
    co_detector.initialize();
    co_detector.add_sensor("Living Room");
    co_detector.add_sensor("Kitchen");
    co_detector.detect_co();
    co_detector.remove_sensor("Living Room");
    co_detector.status();
}

pub struct SmartHomeCODetect {
    sensors: Vec<String>,
    detected: bool,
}

impl SmartHomeCODetect {
    pub fn new() -> Self {
        SmartHomeCODetect {
            sensors: Vec::new(),
            detected: false,
        }
    }

    pub fn initialize(&mut self) {
        // Initialize the CO detector system
        println!("CO Detector System Initialized");
    }

    pub fn add_sensor(&mut self, location: &str) {
        // Add a new sensor to the system
        self.sensors.push(location.to_string());
        println!("Sensor added at {}", location);
    }

    pub fn remove_sensor(&mut self, location: &str) {
        // Remove a sensor from the system
        if let Some(index) = self.sensors.iter().position(|s| s == location) {
            self.sensors.remove(index);
            println!("Sensor removed from {}", location);
        } else {
            println!("No sensor found at {}", location);
        }
    }

    pub fn detect_co(&mut self) {
        // Simulate CO detection
        self.detected = true;
        if self.detected {
            println!("CO detected!");
        } else {
            println!("No CO detected.");
        }
    }

    pub fn status(&self) {
        // Print the current status of the system
        println!("Sensors: {:?}", self.sensors);
        println!("Detected: {}", self.detected);
    }
}
