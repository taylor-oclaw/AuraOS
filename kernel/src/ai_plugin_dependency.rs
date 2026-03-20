extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AiPluginDependency {
    name: String,
    version: String,
    dependencies: Vec<String>,
    enabled: bool,
    config: Vec<(String, String)>,
}

impl AiPluginDependency {
    pub fn new(name: &str, version: &str) -> Self {
        AiPluginDependency {
            name: String::from(name),
            version: String::from(version),
            dependencies: Vec::new(),
            enabled: false,
            config: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(String::from(dependency));
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_config(&mut self, key: &str, value: &str) {
        self.config.push((String::from(key), String::from(value)));
    }

    pub fn get_config(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.config {
            if k == key {
                return Some(v);
            }
        }
        None
    }
}
