extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_child_adapt_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_child_adapt_exit() {
    // Cleanup logic for the module
}

pub struct SpeechChildAdapt {
    data: Vec<u8>,
    name: String,
    enabled: bool,
}

impl SpeechChildAdapt {
    pub fn new(name: &str) -> Self {
        SpeechChildAdapt {
            data: Vec::new(),
            name: String::from(name),
            enabled: true,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn add_data(&mut self, data: &[u8]) {
        if self.enabled {
            self.data.extend_from_slice(data);
        }
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_child_adapt() {
        let mut adapt = SpeechChildAdapt::new("TestModule");
        assert_eq!(adapt.get_name(), "TestModule");
        assert!(adapt.is_enabled());

        adapt.disable();
        assert!(!adapt.is_enabled());
        adapt.enable();
        assert!(adapt.is_enabled());

        let data = [1, 2, 3];
        adapt.add_data(&data);
        assert_eq!(adapt.get_data(), &[1, 2, 3]);

        adapt.clear_data();
        assert!(adapt.get_data().is_empty());

        adapt.set_name("NewName");
        assert_eq!(adapt.get_name(), "NewName");
    }
}
