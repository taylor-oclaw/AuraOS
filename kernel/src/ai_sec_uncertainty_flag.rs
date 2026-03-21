extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AINativeKernelModule {
    name: String,
    version: u32,
    features: Vec<String>,
    enabled: bool,
    uncertainty_level: f32,
}

impl AINativeKernelModule {
    pub fn new(name: &str, version: u32) -> Self {
        AINativeKernelModule {
            name: String::from(name),
            version,
            features: Vec::new(),
            enabled: false,
            uncertainty_level: 0.5,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
    }

    pub fn remove_feature(&mut self, feature: &str) {
        self.features.retain(|f| f != feature);
    }

    pub fn set_uncertainty_level(&mut self, level: f32) {
        if (0.0..=1.0).contains(&level) {
            self.uncertainty_level = level;
        }
    }

    pub fn get_status(&self) -> String {
        let status = if self.enabled { "Enabled" } else { "Disabled" };
        format!("Module: {}, Version: {}, Status: {}, Uncertainty Level: {}",
                self.name, self.version, status, self.uncertainty_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = AINativeKernelModule::new("AI-Sec", 1);
        assert_eq!(module.name, "AI-Sec");
        assert_eq!(module.version, 1);
        assert!(!module.enabled);
        assert_eq!(module.uncertainty_level, 0.5);
    }

    #[test]
    fn test_enable_disable() {
        let mut module = AINativeKernelModule::new("AI-Sec", 1);
        module.enable();
        assert!(module.enabled);
        module.disable();
        assert!(!module.enabled);
    }

    #[test]
    fn test_feature_management() {
        let mut module = AINativeKernelModule::new("AI-Sec", 1);
        module.add_feature("Feature1");
        assert_eq!(module.features.len(), 1);
        module.remove_feature("Feature1");
        assert_eq!(module.features.len(), 0);
    }

    #[test]
    fn test_uncertainty_level() {
        let mut module = AINativeKernelModule::new("AI-Sec", 1);
        module.set_uncertainty_level(0.75);
        assert_eq!(module.uncertainty_level, 0.75);
        module.set_uncertainty_level(2.0); // Invalid level
        assert_eq!(module.uncertainty_level, 0.75);
    }

    #[test]
    fn test_get_status() {
        let mut module = AINativeKernelModule::new("AI-Sec", 1);
        module.enable();
        let status = module.get_status();
        assert_eq!(status, "Module: AI-Sec, Version: 1, Status: Enabled, Uncertainty Level: 0.5");
    }
}
