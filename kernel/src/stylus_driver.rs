extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct StylusDriver {
    device_id: u32,
    pressure_sensitivity: u8,
    tilt_support: bool,
    buttons: Vec<bool>,
    events: Vec<String>,
}

impl StylusDriver {
    pub fn new(device_id: u32) -> Self {
        StylusDriver {
            device_id,
            pressure_sensitivity: 100,
            tilt_support: true,
            buttons: vec![false, false],
            events: Vec::new(),
        }
    }

    pub fn set_pressure_sensitivity(&mut self, sensitivity: u8) {
        if sensitivity > 0 && sensitivity <= 255 {
            self.pressure_sensitivity = sensitivity;
        }
    }

    pub fn get_pressure_sensitivity(&self) -> u8 {
        self.pressure_sensitivity
    }

    pub fn enable_tilt_support(&mut self, enable: bool) {
        self.tilt_support = enable;
    }

    pub fn is_tilt_supported(&self) -> bool {
        self.tilt_support
    }

    pub fn press_button(&mut self, button_id: usize) {
        if button_id < self.buttons.len() {
            self.buttons[button_id] = true;
            self.events.push(String::from("info"));
        }
    }

    pub fn release_button(&mut self, button_id: usize) {
        if button_id < self.buttons.len() {
            self.buttons[button_id] = false;
            self.events.push(String::from("info"));
        }
    }

    pub fn get_events(&self) -> Vec<String> {
        self.events.clone()
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
