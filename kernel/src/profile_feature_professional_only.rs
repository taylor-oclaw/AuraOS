extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ProfileFeatureProfessionalOnly {
    name: String,
    features: Vec<String>,
    active: bool,
}

impl ProfileFeatureProfessionalOnly {
    pub fn new(name: &str) -> Self {
        ProfileFeatureProfessionalOnly {
            name: String::from(name),
            features: Vec::new(),
            active: false,
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

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }
}
