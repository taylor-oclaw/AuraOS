extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentCanaryRelease {
    version: String,
    features: Vec<String>,
    status: String,
    logs: Vec<String>,
    config: Vec<(String, String)>,
}

impl AgentCanaryRelease {
    pub fn new(version: &str) -> Self {
        AgentCanaryRelease {
            version: String::from(version),
            features: Vec::new(),
            status: String::from("inactive"),
            logs: Vec::new(),
            config: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.status = String::from("active");
        self.log(String::from("Agent activated"));
    }

    pub fn deactivate(&mut self) {
        self.status = String::from("inactive");
        self.log(String::from("Agent deactivated"));
    }

    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(String::from(feature));
        self.log(String::from("info"));
    }

    pub fn remove_feature(&mut self, feature: &str) {
        if let Some(index) = self.features.iter().position(|f| f == feature) {
            self.features.remove(index);
            self.log(String::from("info"));
        }
    }

    pub fn log(&mut self, message: String) {
        self.logs.push(message);
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }

    pub fn set_config(&mut self, key: &str, value: &str) {
        if let Some(index) = self.config.iter().position(|(k, _)| k == key) {
            self.config[index].1 = String::from(value);
        } else {
            self.config.push((String::from(key), String::from(value)));
        }
        self.log(String::from("info"));
    }

    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.iter().find_map(|(k, v)| if k == key { Some(v.as_str()) } else { None })
    }
}
