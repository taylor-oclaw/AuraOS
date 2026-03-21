extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct Person {
    name: String,
    age: u8,
    email: String,
    phone_number: String,
    address: String,
}

impl Person {
    pub fn new(name: &str, age: u8, email: &str, phone_number: &str, address: &str) -> Self {
        Person {
            name: String::from(name),
            age,
            email: String::from(email),
            phone_number: String::from(phone_number),
            address: String::from(address),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = String::from(email);
    }

    pub fn get_phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn set_phone_number(&mut self, phone_number: &str) {
        self.phone_number = String::from(phone_number);
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn set_address(&mut self, address: &str) {
        self.address = String::from(address);
    }
}
