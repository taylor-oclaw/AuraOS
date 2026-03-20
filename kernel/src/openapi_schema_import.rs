extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct OpenAPISchema {
    title: String,
    description: String,
    properties: Vec<Property>,
}

impl OpenAPISchema {
    pub fn new(title: &str, description: &str) -> Self {
        OpenAPISchema {
            title: String::from(title),
            description: String::from(description),
            properties: Vec::new(),
        }
    }

    pub fn add_property(&mut self, name: &str, property_type: &str) {
        let prop = Property {
            name: String::from(name),
            property_type: String::from(property_type),
        };
        self.properties.push(prop);
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_properties(&self) -> &[Property] {
        &self.properties
    }
}

#[repr(C)]
pub struct Property {
    name: String,
    property_type: String,
}

impl Property {
    pub fn new(name: &str, property_type: &str) -> Self {
        Property {
            name: String::from(name),
            property_type: String::from(property_type),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_property_type(&self) -> &str {
        &self.property_type
    }
}
