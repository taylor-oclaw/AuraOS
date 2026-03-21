extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct CompatLinuxSysfs {
    name: String,
    attributes: Vec<String>,
}

impl CompatLinuxSysfs {
    pub fn new(name: &str) -> Self {
        CompatLinuxSysfs {
            name: String::from(name),
            attributes: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, attribute: &str) {
        self.attributes.push(String::from(attribute));
    }

    pub fn remove_attribute(&mut self, attribute: &str) -> bool {
        if let Some(index) = self.attributes.iter().position(|attr| attr == attribute) {
            self.attributes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_attributes(&self) -> &[String] {
        &self.attributes
    }

    pub fn has_attribute(&self, attribute: &str) -> bool {
        self.attributes.contains(&String::from(attribute))
    }

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }
}
