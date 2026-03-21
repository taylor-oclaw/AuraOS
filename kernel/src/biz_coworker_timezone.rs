extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct CoworkerLocation {
    name: String,
    coordinates: (i32, i32),
    coworkers: Vec<String>,
}

impl CoworkerLocation {
    pub fn new(name: &str, x: i32, y: i32) -> Self {
        CoworkerLocation {
            name: String::from(name),
            coordinates: (x, y),
            coworkers: Vec::new(),
        }
    }

    pub fn add_coworker(&mut self, coworker_name: &str) {
        self.coworkers.push(String::from(coworker_name));
    }

    pub fn remove_coworker(&mut self, coworker_name: &str) -> bool {
        if let Some(index) = self.coworkers.iter().position(|c| c == coworker_name) {
            self.coworkers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_coworkers(&self) -> &Vec<String> {
        &self.coworkers
    }

    pub fn set_coordinates(&mut self, x: i32, y: i32) {
        self.coordinates = (x, y);
    }

    pub fn get_location_info(&self) -> String {
        let mut info = String::from("info")", self.name, self.coordinates.0, self.coordinates.1;
        if !self.coworkers.is_empty() {
            info.push_str(", Coworkers: ");
            for (i, coworker) in self.coworkers.iter().enumerate() {
                if i > 0 {
                    info.push(',');
                }
                info.push_str(coworker);
            }
        }
        info
    }
}
