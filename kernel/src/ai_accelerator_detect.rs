extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AiAcceleratorDetect {
    accelerator_type: u32,
}

impl AiAcceleratorDetect {
    pub fn new() -> Self {
        Self { accelerator_type: 0 }
    }

    pub fn detect_accelerator(&mut self) -> bool {
        // Simulate a detection of an AI accelerator
        self.accelerator_type = 1;
        true
    }

    pub fn get_accelerator_type(&self) -> u32 {
        self.accelerator_type
    }

    pub fn is_accelerator_detected(&self) -> bool {
        self.accelerator_type != 0
    }

    pub fn reset_accelerator_detection(&mut self) {
        self.accelerator_type = 0;
    }
}

pub struct AiAcceleratorInfo {
    accelerator_name: String,
    accelerator_version: u32,
}

impl AiAcceleratorInfo {
    pub fn new(accelerator_name: &str, accelerator_version: u32) -> Self {
        Self {
            accelerator_name: String::from(accelerator_name),
            accelerator_version,
        }
    }

    pub fn get_accelerator_name(&self) -> &str {
        self.accelerator_name.as_str()
    }

    pub fn get_accelerator_version(&self) -> u32 {
        self.accelerator_version
    }
}