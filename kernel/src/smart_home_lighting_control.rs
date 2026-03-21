extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SmartHomeLightingControl {
    lights: Vec<Light>,
}

impl SmartHomeLightingControl {
    pub fn new() -> Self {
        SmartHomeLightingControl { lights: Vec::new() }
    }

    pub fn add_light(&mut self, name: String, brightness: u8) {
        let light = Light { name, brightness };
        self.lights.push(light);
    }

    pub fn remove_light_by_name(&mut self, name: &str) -> bool {
        if let Some(index) = self.lights.iter().position(|light| light.name == name) {
            self.lights.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_brightness(&mut self, name: &str, brightness: u8) -> bool {
        if let Some(light) = self.lights.iter_mut().find(|light| light.name == name) {
            light.brightness = brightness;
            true
        } else {
            false
        }
    }

    pub fn get_brightness(&self, name: &str) -> Option<u8> {
        self.lights.iter().find_map(|light| {
            if light.name == name {
                Some(light.brightness)
            } else {
                None
            }
        })
    }

    pub fn list_lights(&self) -> Vec<String> {
        self.lights.iter().map(|light| light.name.clone()).collect()
    }
}

#[derive(Debug)]
struct Light {
    name: String,
    brightness: u8,
}
