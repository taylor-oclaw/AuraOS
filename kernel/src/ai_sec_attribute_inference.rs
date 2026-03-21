extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn ai_sec_attribute_inference_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn ai_sec_attribute_inference_exit() {
    // Cleanup code for the module
}

pub struct AIAttributeInference {
    attributes: Vec<String>,
}

impl AIAttributeInference {
    pub fn new() -> Self {
        AIAttributeInference {
            attributes: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, attribute: String) {
        self.attributes.push(attribute);
    }

    pub fn remove_attribute(&mut self, attribute: &str) -> bool {
        if let Some(index) = self.attributes.iter().position(|x| x == attribute) {
            self.attributes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_attribute(&self, attribute: &str) -> bool {
        self.attributes.contains(&String::from(attribute))
    }

    pub fn get_attributes(&self) -> Vec<String> {
        self.attributes.clone()
    }

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_attribute_inference() {
        let mut inference = AIAttributeInference::new();

        assert_eq!(inference.get_attributes().len(), 0);

        inference.add_attribute(String::from("secure"));
        inference.add_attribute(String::from("confidential"));

        assert_eq!(inference.get_attributes().len(), 2);
        assert!(inference.has_attribute("secure"));
        assert!(inference.has_attribute("confidential"));

        assert!(inference.remove_attribute("secure"));
        assert!(!inference.has_attribute("secure"));
        assert_eq!(inference.get_attributes().len(), 1);

        inference.clear_attributes();
        assert_eq!(inference.get_attributes().len(), 0);
    }
}
