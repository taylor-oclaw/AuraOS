extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct ProfileWFHHybridSchedule {
    name: String,
    work_hours: Vec<u8>,
    flex_hours: Vec<u8>,
}

impl ProfileWFHHybridSchedule {
    pub fn new(name: &str, work_hours: &[u8], flex_hours: &[u8]) -> Self {
        ProfileWFHHybridSchedule {
            name: String::from(name),
            work_hours: Vec::from(work_hours),
            flex_hours: Vec::from(flex_hours),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn add_work_hour(&mut self, hour: u8) {
        if !self.work_hours.contains(&hour) && hour < 24 {
            self.work_hours.push(hour);
        }
    }

    pub fn remove_work_hour(&mut self, hour: u8) {
        if let Some(index) = self.work_hours.iter().position(|&h| h == hour) {
            self.work_hours.remove(index);
        }
    }

    pub fn add_flex_hour(&mut self, hour: u8) {
        if !self.flex_hours.contains(&hour) && hour < 24 {
            self.flex_hours.push(hour);
        }
    }

    pub fn remove_flex_hour(&mut self, hour: u8) {
        if let Some(index) = self.flex_hours.iter().position(|&h| h == hour) {
            self.flex_hours.remove(index);
        }
    }

    pub fn get_work_hours(&self) -> &[u8] {
        &self.work_hours
    }

    pub fn get_flex_hours(&self) -> &[u8] {
        &self.flex_hours
    }
}
