extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AccessDyslexiaFont {
    font_data: Vec<u8>,
    font_name: String,
    version: String,
    author: String,
    license: String,
}

impl AccessDyslexiaFont {
    pub fn new(font_data: Vec<u8>, font_name: &str, version: &str, author: &str, license: &str) -> Self {
        AccessDyslexiaFont {
            font_data,
            font_name: String::from(font_name),
            version: String::from(version),
            author: String::from(author),
            license: String::from(license),
        }
    }

    pub fn get_font_data(&self) -> &Vec<u8> {
        &self.font_data
    }

    pub fn get_font_name(&self) -> &str {
        &self.font_name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn get_license(&self) -> &str {
        &self.license
    }
}
