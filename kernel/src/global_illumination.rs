extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GlobalIllumination {
    lights: Vec<String>,
    objects: Vec<String>,
    environment_map: String,
}

impl GlobalIllumination {
    pub fn new() -> Self {
        GlobalIllumination {
            lights: Vec::new(),
            objects: Vec::new(),
            environment_map: String::from("default_env_map"),
        }
    }

    pub fn add_light(&mut self, light_name: &str) {
        self.lights.push(String::from(light_name));
    }

    pub fn remove_light(&mut self, light_name: &str) -> bool {
        if let Some(index) = self.lights.iter().position(|x| x == light_name) {
            self.lights.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_object(&mut self, object_name: &str) {
        self.objects.push(String::from(object_name));
    }

    pub fn remove_object(&mut self, object_name: &str) -> bool {
        if let Some(index) = self.objects.iter().position(|x| x == object_name) {
            self.objects.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_environment_map(&mut self, map_name: &str) {
        self.environment_map = String::from(map_name);
    }

    pub fn get_lights(&self) -> Vec<String> {
        self.lights.clone()
    }

    pub fn get_objects(&self) -> Vec<String> {
        self.objects.clone()
    }

    pub fn get_environment_map(&self) -> String {
        self.environment_map.clone()
    }
}
