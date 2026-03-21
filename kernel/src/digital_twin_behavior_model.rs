extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct DigitalTwinBehaviorModel {
    name: String,
    attributes: Vec<(String, String)>,
    behaviors: Vec<String>,
}

impl DigitalTwinBehaviorModel {
    pub fn new(name: &str) -> Self {
        DigitalTwinBehaviorModel {
            name: String::from(name),
            attributes: Vec::new(),
            behaviors: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.push((String::from(key), String::from(value)));
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn add_behavior(&mut self, behavior: &str) {
        self.behaviors.push(String::from(behavior));
    }

    pub fn get_behaviors(&self) -> &[String] {
        &self.behaviors
    }

    pub fn remove_attribute(&mut self, key: &str) {
        self.attributes.retain(|k, _| k != key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_twin_behavior_model() {
        let mut model = DigitalTwinBehaviorModel::new("TestModel");
        assert_eq!(model.name, "TestModel");

        model.add_attribute("temperature", "25C");
        assert_eq!(model.get_attribute("temperature"), Some(&String::from("25C")));

        model.add_behavior("monitor_temperature");
        assert_eq!(model.get_behaviors(), &vec![String::from("monitor_temperature")]);

        model.remove_attribute("temperature");
        assert_eq!(model.get_attribute("temperature"), None);
    }
}
