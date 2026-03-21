extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn travel_offline_map_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn travel_offline_map_exit() {
    // Cleanup logic for the module
}

pub struct TravelOfflineMap {
    map_data: Vec<u8>,
    current_location: String,
    destinations: Vec<String>,
    route: Vec<String>,
}

impl TravelOfflineMap {
    pub fn new(map_data: Vec<u8>) -> Self {
        TravelOfflineMap {
            map_data,
            current_location: String::from("Unknown"),
            destinations: Vec::new(),
            route: Vec::new(),
        }
    }

    pub fn set_current_location(&mut self, location: &str) {
        self.current_location = String::from(location);
    }

    pub fn add_destination(&mut self, destination: &str) {
        self.destinations.push(String::from(destination));
    }

    pub fn find_route(&self, start: &str, end: &str) -> Vec<String> {
        // Dummy route finding logic
        let mut route = Vec::new();
        route.push(start.to_string());
        route.push(end.to_string());
        route
    }

    pub fn get_current_location(&self) -> &String {
        &self.current_location
    }

    pub fn list_destinations(&self) -> &Vec<String> {
        &self.destinations
    }
}

#[no_mangle]
pub extern "C" fn travel_offline_map_create(map_data: *const u8, map_size: usize) -> *mut TravelOfflineMap {
    let map_data_vec = unsafe { Vec::from_raw_parts(map_data as *mut u8, map_size, map_size) };
    Box::into_raw(Box::new(TravelOfflineMap::new(map_data_vec)))
}

#[no_mangle]
pub extern "C" fn travel_offline_map_destroy(map: *mut TravelOfflineMap) {
    unsafe { drop(Box::from_raw(map)) }
}
