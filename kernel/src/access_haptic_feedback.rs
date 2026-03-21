extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct HapticFeedbackDevice {
    intensity: u8,
    duration: u16,
    pattern: Vec<u8>,
    is_enabled: bool,
}

impl HapticFeedbackDevice {
    pub fn new(intensity: u8, duration: u16) -> Self {
        HapticFeedbackDevice {
            intensity,
            duration,
            pattern: Vec::new(),
            is_enabled: true,
        }
    }

    pub fn set_intensity(&mut self, intensity: u8) {
        if intensity <= 100 {
            self.intensity = intensity;
        }
    }

    pub fn get_intensity(&self) -> u8 {
        self.intensity
    }

    pub fn set_duration(&mut self, duration: u16) {
        self.duration = duration;
    }

    pub fn get_duration(&self) -> u16 {
        self.duration
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_pattern(&mut self, pattern: Vec<u8>) {
        self.pattern = pattern;
    }

    pub fn get_pattern(&self) -> &Vec<u8> {
        &self.pattern
    }
}

#[no_mangle]
pub extern "C" fn create_haptic_device(intensity: u8, duration: u16) -> *mut HapticFeedbackDevice {
    Box::into_raw(Box::new(HapticFeedbackDevice::new(intensity, duration)))
}

#[no_mangle]
pub extern "C" fn destroy_haptic_device(device: *mut HapticFeedbackDevice) {
    if device.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(device)) };
}
