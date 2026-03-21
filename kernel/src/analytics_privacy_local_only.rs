extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AnalyticsPrivacyLocalOnly {
    data: Vec<String>,
    enabled: bool,
}

impl AnalyticsPrivacyLocalOnly {
    pub fn new() -> Self {
        AnalyticsPrivacyLocalOnly {
            data: Vec::new(),
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_data(&mut self, data: String) {
        if self.enabled {
            self.data.push(data);
        }
    }

    pub fn get_data(&self) -> Vec<String> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let aplo = AnalyticsPrivacyLocalOnly::new();
        assert!(aplo.enabled);
        assert_eq!(aplo.data.len(), 0);
    }

    #[test]
    fn test_enable_disable() {
        let mut aplo = AnalyticsPrivacyLocalOnly::new();
        aplo.disable();
        assert!(!aplo.is_enabled());
        aplo.enable();
        assert!(aplo.is_enabled());
    }

    #[test]
    fn test_add_data() {
        let mut aplo = AnalyticsPrivacyLocalOnly::new();
        aplo.add_data(String::from("test1"));
        aplo.add_data(String::from("test2"));
        assert_eq!(aplo.data.len(), 2);
    }

    #[test]
    fn test_get_data() {
        let mut aplo = AnalyticsPrivacyLocalOnly::new();
        aplo.add_data(String::from("test1"));
        aplo.add_data(String::from("test2"));
        let data = aplo.get_data();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], "test1");
        assert_eq!(data[1], "test2");
    }
}
