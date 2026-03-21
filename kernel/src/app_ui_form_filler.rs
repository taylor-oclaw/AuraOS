extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppUIFormFiller {
    fields: Vec<(String, String)>,
}

impl AppUIFormFiller {
    pub fn new() -> Self {
        AppUIFormFiller {
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, name: &str, value: &str) {
        self.fields.push((String::from(name), String::from(value)));
    }

    pub fn get_field_value(&self, name: &str) -> Option<&String> {
        self.fields.iter().find_map(|(field_name, field_value)| {
            if field_name == name {
                Some(field_value)
            } else {
                None
            }
        }
    }

    pub fn remove_field(&mut self, name: &str) {
        self.fields.retain(|(field_name, _)| field_name != name);
    }

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    pub fn list_fields(&self) -> Vec<&String> {
        self.fields.iter().map(|(_, value)| value).collect()
    }
}
