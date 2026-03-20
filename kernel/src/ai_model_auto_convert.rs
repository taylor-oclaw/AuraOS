extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct AiModelAutoConvert {
    model_name: String,
    supported_formats: Vec<String>,
    conversion_log: Vec<String>,
}

impl AiModelAutoConvert {
    pub fn new(model_name: &str, supported_formats: &[&str]) -> Self {
        let mut formats = Vec::new();
        for format in supported_formats {
            formats.push(String::from(*format));
        }
        AiModelAutoConvert {
            model_name: String::from(model_name),
            supported_formats: formats,
            conversion_log: Vec::new(),
        }
    }

    pub fn add_format(&mut self, format: &str) {
        if !self.supported_formats.contains(&String::from(format)) {
            self.supported_formats.push(String::from(format));
        }
    }

    pub fn remove_format(&mut self, format: &str) {
        self.supported_formats.retain(|f| f != format);
    }

    pub fn is_format_supported(&self, format: &str) -> bool {
        self.supported_formats.contains(&String::from(format))
    }

    pub fn log_conversion(&mut self, from_format: &str, to_format: &str) {
        let log_entry = format!("Converted from {} to {}", from_format, to_format);
        self.conversion_log.push(log_entry);
    }

    pub fn get_conversion_log(&self) -> Vec<String> {
        self.conversion_log.clone()
    }
}
