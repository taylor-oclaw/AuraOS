extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ProfileModeCore {
    name: String,
    version: u32,
    features: Vec<String>,
    active: bool,
    config: Vec<(String, String)>,
}

impl ProfileModeCore {
    pub fn new(name: &str, version: u32) -> Self {
        ProfileModeCore {
            name: String::from(name),
            version,
            features: Vec::new(),
            active: false,
            config: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn remove_feature(&mut self, feature: &str) {
        self.features.retain(|f| f != feature);
    }

    pub fn set_config(&mut self, key: &str, value: &str) {
        let key = String::from(key);
        let value = String::from(value);
        match self.config.iter().position(|(k, _)| k == key) {
            Some(index) => self.config[index] = (key, value),
            None => self.config.push((key, value)),
        }
    }

    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }
}
