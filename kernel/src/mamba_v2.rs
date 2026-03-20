extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mamba_v2_init() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn mamba_v2_exit() -> i32 {
    0
}

pub struct MambaV2 {
    name: String,
    version: u32,
    features: Vec<String>,
    active: bool,
}

impl MambaV2 {
    pub fn new(name: &str, version: u32) -> Self {
        MambaV2 {
            name: String::from(name),
            version,
            features: Vec::new(),
            active: false,
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

    pub fn remove_feature(&mut self, feature: &str) -> bool {
        if let Some(index) = self.features.iter().position(|f| f == feature) {
            self.features.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_features(&self) -> Vec<String> {
        self.features.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mamba_v2() {
        let mut mamba = MambaV2::new("Mamba V2", 1);
        assert_eq!(mamba.name, "Mamba V2");
        assert_eq!(mamba.version, 1);
        assert!(!mamba.active);

        mamba.activate();
        assert!(mamba.active);

        mamba.deactivate();
        assert!(!mamba.active);

        mamba.add_feature("AI Integration");
        mamba.add_feature("Secure Boot");
        assert_eq!(mamba.list_features(), vec![String::from("AI Integration"), String::from("Secure Boot")]);

        assert!(mamba.remove_feature("AI Integration"));
        assert_eq!(mamba.list_features(), vec![String::from("Secure Boot")]);
        assert!(!mamba.remove_feature("AI Integration"));
    }
}
