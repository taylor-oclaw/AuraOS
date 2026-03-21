extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
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

pub struct PeopleDetailCapture {
    people: Vec<Person>,
}

impl PeopleDetailCapture {
    pub fn new() -> Self {
        PeopleDetailCapture {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn find_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.get_name() == name)
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|p| p.get_name() != name);
    }
}
