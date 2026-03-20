extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Initialize the compass driver module
    let mut compass = CompassDriver::new();
    
    // Example usage of the compass driver methods
    compass.calibrate();
    compass.set_heading(90);
    let heading = compass.get_heading();
    println!("Current Heading: {}", heading);
    compass.enable_continuous_mode(true);
    compass.disable_continuous_mode(false);
}

pub struct CompassDriver {
    heading: u16,
    is_calibrated: bool,
    continuous_mode: bool,
}

impl CompassDriver {
    pub fn new() -> Self {
        CompassDriver {
            heading: 0,
            is_calibrated: false,
            continuous_mode: false,
        }
    }

    pub fn calibrate(&mut self) {
        // Simulate calibration process
        self.is_calibrated = true;
        println!("Compass calibrated successfully.");
    }

    pub fn set_heading(&mut self, heading: u16) {
        if self.is_calibrated {
            self.heading = heading % 360; // Ensure heading is within 0-359 range
            println!("Heading set to {} degrees.", self.heading);
        } else {
            println!("Compass must be calibrated before setting heading.");
        }
    }

    pub fn get_heading(&self) -> u16 {
        self.heading
    }

    pub fn enable_continuous_mode(&mut self, enable: bool) {
        self.continuous_mode = enable;
        if enable {
            println!("Continuous mode enabled.");
        } else {
            println!("Continuous mode disabled.");
        }
    }

    pub fn disable_continuous_mode(&mut self, disable: bool) {
        self.enable_continuous_mode(!disable);
    }
}
