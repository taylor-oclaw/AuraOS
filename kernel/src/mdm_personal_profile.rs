extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mdm_personal_profile_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mdm_personal_profile_exit() {
    // Cleanup logic for the module
}

pub struct PersonalProfile {
    name: String,
    age: u32,
    email: String,
    phone_number: String,
    address: String,
}

impl PersonalProfile {
    pub fn new(name: &str, age: u32, email: &str, phone_number: &str, address: &str) -> Self {
        PersonalProfile {
            name: String::from(name),
            age,
            email: String::from(email),
            phone_number: String::from(phone_number),
            address: String::from(address),
        }
    }

    pub fn update_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn update_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn update_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }

    pub fn update_phone_number(&mut self, new_phone_number: &str) {
        self.phone_number = String::from(new_phone_number);
    }

    pub fn update_address(&mut self, new_address: &str) {
        self.address = String::from(new_address);
    }
}
