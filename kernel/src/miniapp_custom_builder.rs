extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn miniapp_custom_builder_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn miniapp_custom_builder_exit() {
    // Cleanup logic for the module
}

pub struct MiniAppCustomBuilder {
    name: String,
    version: u32,
    features: Vec<String>,
    dependencies: Vec<String>,
    description: String,
}

impl MiniAppCustomBuilder {
    pub fn new(name: &str, version: u32) -> Self {
        MiniAppCustomBuilder {
            name: String::from(name),
            version,
            features: Vec::new(),
            dependencies: Vec::new(),
            description: String::new(),
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn remove_feature(&mut self, feature: &str) {
        if let Some(index) = self.features.iter().position(|f| f == feature) {
            self.features.remove(index);
        }
    }

    pub fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(String::from(dependency));
    }

    pub fn remove_dependency(&mut self, dependency: &str) {
        if let Some(index) = self.dependencies.iter().position(|d| d == dependency) {
            self.dependencies.remove(index);
        }
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_features(&self) -> &[String] {
        &self.features
    }

    pub fn get_dependencies(&self) -> &[String] {
        &self.dependencies
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
