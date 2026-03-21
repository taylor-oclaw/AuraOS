extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct PeopleDetailPet {
    name: String,
    age: u32,
    pet_name: Option<String>,
    hobbies: Vec<String>,
}

impl PeopleDetailPet {
    pub fn new(name: &str, age: u32) -> Self {
        PeopleDetailPet {
            name: String::from(name),
            age,
            pet_name: None,
            hobbies: Vec::new(),
        }
    }

    pub fn set_pet_name(&mut self, pet_name: &str) {
        self.pet_name = Some(String::from(pet_name));
    }

    pub fn add_hobby(&mut self, hobby: &str) {
        self.hobbies.push(String::from(hobby));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn get_pet_name(&self) -> Option<&str> {
        self.pet_name.as_deref()
    }

    pub fn list_hobbies(&self) -> &[String] {
        &self.hobbies
    }
}
