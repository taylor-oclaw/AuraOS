extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn digital_twin_user_init() {
    // Initialization logic for the module
}

pub extern "C" fn digital_twin_user_exit() {
    // Cleanup logic for the module
}

pub struct DigitalTwinUser {
    name: String,
    attributes: Vec<String>,
    status: String,
}

impl DigitalTwinUser {
    pub fn new(name: &str) -> Self {
        DigitalTwinUser {
            name: String::from(name),
            attributes: Vec::new(),
            status: String::from("active"),
        }
    }

    pub fn add_attribute(&mut self, attribute: &str) {
        self.attributes.push(String::from(attribute));
    }

    pub fn remove_attribute(&mut self, attribute: &str) -> bool {
        if let Some(index) = self.attributes.iter().position(|x| x == attribute) {
            self.attributes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_attributes(&self) -> &[String] {
        &self.attributes
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_twin_user() {
        let mut user = DigitalTwinUser::new("Alice");
        assert_eq!(user.get_name(), "Alice");
        assert_eq!(user.get_status(), "active");

        user.add_attribute("admin");
        assert_eq!(user.get_attributes().len(), 1);
        assert_eq!(user.get_attributes()[0], "admin");

        let removed = user.remove_attribute("admin");
        assert!(removed);
        assert_eq!(user.get_attributes().len(), 0);

        user.set_status("inactive");
        assert_eq!(user.get_status(), "inactive");
    }
}
