extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn car_android_auto_compat_init() {
    // Initialization logic for the module
}

pub extern "C" fn car_android_auto_compat_exit() {
    // Cleanup logic for the module
}

pub struct CarAndroidAutoCompat {
    models: Vec<String>,
    features: Vec<String>,
    version: String,
    is_enabled: bool,
}

impl CarAndroidAutoCompat {
    pub fn new(version: &str) -> Self {
        CarAndroidAutoCompat {
            models: Vec::new(),
            features: Vec::new(),
            version: String::from(version),
            is_enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn add_model(&mut self, model: &str) {
        self.models.push(String::from(model));
    }

    pub fn remove_model(&mut self, model: &str) -> bool {
        if let Some(index) = self.models.iter().position(|m| m == model) {
            self.models.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn remove_feature(&mut self, feature: &str) -> bool {
        if let Some(index) = self.features.iter().position(|f| f == feature) {
            self.features.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_model_supported(&self, model: &str) -> bool {
        self.models.contains(&String::from(model))
    }

    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.features.contains(&String::from(feature))
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn is_module_enabled(&self) -> bool {
        self.is_enabled
    }
}
