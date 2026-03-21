extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct ProfileCommuteHandsFree {
    user_id: u32,
    commute_routes: Vec<String>,
    preferred_transport: String,
    travel_time_stats: Vec<u32>,
    notifications_enabled: bool,
}

impl ProfileCommuteHandsFree {
    pub fn new(user_id: u32) -> Self {
        ProfileCommuteHandsFree {
            user_id,
            commute_routes: Vec::new(),
            preferred_transport: String::from(""),
            travel_time_stats: Vec::new(),
            notifications_enabled: false,
        }
    }

    pub fn add_route(&mut self, route: &str) {
        self.commute_routes.push(String::from(route));
    }

    pub fn set_preferred_transport(&mut self, transport: &str) {
        self.preferred_transport = String::from(transport);
    }

    pub fn record_travel_time(&mut self, time: u32) {
        self.travel_time_stats.push(time);
    }

    pub fn enable_notifications(&mut self) {
        self.notifications_enabled = true;
    }

    pub fn disable_notifications(&mut self) {
        self.notifications_enabled = false;
    }
}
