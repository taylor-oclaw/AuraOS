extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct IccProfile {
    name: String,
    version: u32,
    color_space: String,
    device_class: String,
    rendering_intent: String,
}

impl IccProfile {
    pub fn new(name: &str, version: u32, color_space: &str, device_class: &str, rendering_intent: &str) -> Self {
        IccProfile {
            name: String::from(name),
            version,
            color_space: String::from(color_space),
            device_class: String::from(device_class),
            rendering_intent: String::from(rendering_intent),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    pub fn get_color_space(&self) -> &str {
        &self.color_space
    }

    pub fn set_color_space(&mut self, color_space: &str) {
        self.color_space = String::from(color_space);
    }

    pub fn get_device_class(&self) -> &str {
        &self.device_class
    }

    pub fn set_device_class(&mut self, device_class: &str) {
        self.device_class = String::from(device_class);
    }

    pub fn get_rendering_intent(&self) -> &str {
        &self.rendering_intent
    }

    pub fn set_rendering_intent(&mut self, rendering_intent: &str) {
        self.rendering_intent = String::from(rendering_intent);
    }
}
