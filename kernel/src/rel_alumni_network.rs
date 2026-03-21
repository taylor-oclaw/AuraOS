extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let network = AlumniNetwork::new();
    network.add_alumni("Alice", 2015);
    network.add_alumni("Bob", 2016);
    network.add_alumni("Charlie", 2017);


    loop {}
}

pub struct AlumniNetwork {
    alumni_list: Vec<(String, u32)>,
}

impl AlumniNetwork {
    pub fn new() -> Self {
        AlumniNetwork {
            alumni_list: Vec::new(),
        }
    }

    pub fn add_alumni(&mut self, name: &str, year: u32) {
        let name = String::from(name);
        self.alumni_list.push((name, year));
    }

    pub fn get_alumni_count(&self) -> usize {
        self.alumni_list.len()
    }

    pub fn is_alumni(&self, name: &str) -> bool {
        self.alumni_list.iter().any(|(alumni_name, _)| alumni_name == name)
    }

    pub fn get_year_of_graduation(&self, name: &str) -> Option<u32> {
        for (alumni_name, year) in &self.alumni_list {
            if alumni_name == name {
                return Some(*year);
            }
        }
        None
    }

    pub fn remove_alumni(&mut self, name: &str) {
        self.alumni_list.retain(|(alumni_name, _)| alumni_name != name);
    }
}
