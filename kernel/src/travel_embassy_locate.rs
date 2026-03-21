extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct TravelEmbassyLocate {
    embassies: Vec<String>,
}

impl TravelEmbassyLocate {
    pub fn new() -> Self {
        TravelEmbassyLocate {
            embassies: Vec::new(),
        }
    }

    pub fn add_embassy(&mut self, embassy_name: &str) {
        self.embassies.push(String::from(embassy_name));
    }

    pub fn remove_embassy(&mut self, embassy_name: &str) -> bool {
        if let Some(index) = self.embassies.iter().position(|e| e == embassy_name) {
            self.embassies.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_embassy_count(&self) -> usize {
        self.embassies.len()
    }

    pub fn list_embassies(&self) -> Vec<String> {
        self.embassies.clone()
    }

    pub fn find_embassy(&self, embassy_name: &str) -> bool {
        self.embassies.contains(&String::from(embassy_name))
    }
}
