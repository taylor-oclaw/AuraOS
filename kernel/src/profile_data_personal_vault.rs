extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct PersonalVault {
    name: String,
    age: u32,
    email: String,
    phone_number: String,
    address: String,
}

impl PersonalVault {
    pub fn new(name: &str, age: u32, email: &str, phone_number: &str, address: &str) -> Self {
        PersonalVault {
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

#[no_mangle]
pub extern "C" fn rust_profile_data_personal_vault_new(
    name: *const u8,
    age: u32,
    email: *const u8,
    phone_number: *const u8,
    address: *const u8,
) -> *mut PersonalVault {
    let name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, 0)) };
    let email_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(email, 0)) };
    let phone_number_str = unsafe {
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(phone_number, 0))
    };
    let address_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(address, 0)) };

    Box::into_raw(Box::new(PersonalVault::new(
        name_str,
        age,
        email_str,
        phone_number_str,
        address_str,
    )))
}

#[no_mangle]
pub extern "C" fn rust_profile_data_personal_vault_free(ptr: *mut PersonalVault) {
    unsafe { drop(Box::from_raw(ptr)) }
}
