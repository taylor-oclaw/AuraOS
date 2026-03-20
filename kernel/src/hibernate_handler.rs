extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HibernateHandler {
    state: String,
    memory_map: Vec<u64>,
    power_state: u8,
}

impl HibernateHandler {
    pub fn new() -> Self {
        HibernateHandler {
            state: String::from("Active"),
            memory_map: Vec::new(),
            power_state: 0, // 0 - Active, 1 - Suspended
        }
    }

    pub fn suspend(&mut self) {
        if self.power_state == 0 {
            self.state = String::from("Suspended");
            self.power_state = 1;
            println!("System suspended.");
        } else {
            println!("System is already suspended.");
        }
    }

    pub fn resume(&mut self) {
        if self.power_state == 1 {
            self.state = String::from("Active");
            self.power_state = 0;
            println!("System resumed.");
        } else {
            println!("System is not in a suspended state.");
        }
    }

    pub fn add_memory_region(&mut self, start: u64, end: u64) {
        self.memory_map.push(start);
        self.memory_map.push(end);
        println!("Memory region added: Start - {}, End - {}", start, end);
    }

    pub fn get_memory_regions(&self) -> &Vec<u64> {
        &self.memory_map
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }
}
