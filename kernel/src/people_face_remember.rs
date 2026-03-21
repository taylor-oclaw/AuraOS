extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn people_face_remember_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn people_face_remember_exit() {
    // Cleanup logic for the module
}

pub struct PeopleFaceRemember {
    names: Vec<String>,
    ages: Vec<u8>,
    faces: Vec<Vec<u8>>, // Assuming face data is represented as a vector of bytes
}

impl PeopleFaceRemember {
    pub fn new() -> Self {
        PeopleFaceRemember {
            names: Vec::new(),
            ages: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: String, age: u8, face_data: Vec<u8>) {
        self.names.push(name);
        self.ages.push(age);
        self.faces.push(face_data);
    }

    pub fn get_person_count(&self) -> usize {
        self.names.len()
    }

    pub fn find_person_by_name(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }

    pub fn get_person_details(&self, index: usize) -> Option<(String, u8, Vec<u8>)> {
        if index < self.names.len() {
            Some((self.names[index].clone(), self.ages[index], self.faces[index].clone()))
        } else {
            None
        }
    }

    pub fn remove_person(&mut self, index: usize) -> Option<(String, u8, Vec<u8>)> {
        if index < self.names.len() {
            Some((
                self.names.remove(index),
                self.ages.remove(index),
                self.faces.remove(index),
            ))
        } else {
            None
        }
    }
}
