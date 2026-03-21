extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct Person {
    name: String,
    communication_style: CommunicationStyle,
}

impl Person {
    pub fn new(name: &str, style: CommunicationStyle) -> Self {
        Person {
            name: String::from(name),
            communication_style: style,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_communication_style(&self) -> CommunicationStyle {
        self.communication_style
    }

    pub fn set_communication_style(&mut self, style: CommunicationStyle) {
        self.communication_style = style;
    }

    pub fn communicate(&self, message: &str) -> String {
        match self.communication_style {
            CommunicationStyle::Direct => String::from("info"),
            CommunicationStyle::Indirect => String::from("info"),
            CommunicationStyle::Polite => String::from("info"),
            CommunicationStyle::Casual => String::from("info"),
            CommunicationStyle::Formal => String::from("info"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum CommunicationStyle {
    Direct,
    Indirect,
    Polite,
    Casual,
    Formal,
}
