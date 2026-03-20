extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraSettings {
    settings: Vec<(String, String)>,
}

impl AuraSettings {
    pub fn new() -> Self {
        AuraSettings {
            settings: Vec::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let key = String::from(key);
        let value = String::from(value);
        match self.settings.iter().position(|(k, _)| k == &key) {
            Some(index) => self.settings[index] = (key, value),
            None => self.settings.push((key, value)),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.settings.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn remove(&mut self, key: &str) {
        self.settings.retain(|(k, _)| k != key);
    }

    pub fn list_keys(&self) -> Vec<&String> {
        self.settings.iter().map(|(k, _)| k).collect()
    }

    pub fn list_values(&self) -> Vec<&String> {
        self.settings.iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_settings() {
        let mut settings = AuraSettings::new();
        settings.set("brightness", "70");
        settings.set("volume", "50");

        assert_eq!(settings.get("brightness"), Some(&String::from("70")));
        assert_eq!(settings.get("volume"), Some(&String::from("50")));

        settings.remove("brightness");
        assert_eq!(settings.get("brightness"), None);

        let keys = settings.list_keys();
        assert_eq!(keys, vec![&String::from("volume")]);

        let values = settings.list_values();
        assert_eq!(values, vec![&String::from("50")]);
    }
}
