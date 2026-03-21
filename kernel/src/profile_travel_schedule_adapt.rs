extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut schedule = ProfileTravelScheduleAdapt::new();
    schedule.add_event("Lunch", 120);
    schedule.add_event("Meeting", 60);
    schedule.add_event("Breakfast", 30);
    schedule.remove_event("Breakfast");
    schedule.update_event_duration("Lunch", 90);
    let total_time = schedule.total_time_spent();
    // Do something with total_time
    loop {}
}

pub struct ProfileTravelScheduleAdapt {
    events: Vec<(String, u32)>,
}

impl ProfileTravelScheduleAdapt {
    pub fn new() -> Self {
        ProfileTravelScheduleAdapt { events: Vec::new() }
    }

    pub fn add_event(&mut self, name: &str, duration: u32) {
        let event_name = String::from(name);
        self.events.push((event_name, duration));
    }

    pub fn remove_event(&mut self, name: &str) {
        self.events.retain(|(event_name, _)| event_name != name);
    }

    pub fn update_event_duration(&mut self, name: &str, new_duration: u32) {
        if let Some((_, duration)) = self.events.iter_mut().find(|(event_name, _)| event_name == name) {
            *duration = new_duration;
        }
    }

    pub fn total_time_spent(&self) -> u32 {
        self.events.iter().map(|(_, duration)| duration).sum()
    }

    pub fn get_event_durations(&self) -> Vec<(String, u32)> {
        self.events.clone()
    }
}
